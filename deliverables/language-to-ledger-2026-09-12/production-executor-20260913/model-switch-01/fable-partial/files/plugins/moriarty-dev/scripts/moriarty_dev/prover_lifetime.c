#define _GNU_SOURCE

#include <errno.h>
#include <fcntl.h>
#include <inttypes.h>
#include <limits.h>
#include <poll.h>
#include <signal.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/prctl.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <time.h>
#include <unistd.h>

#ifndef O_NOFOLLOW
#error "prover_lifetime requires O_NOFOLLOW"
#endif

#define CONTROL_MAX_BYTES 512U
#define OWN_RUNTIME_NS UINT64_C(1500000000000)
#define SIGNAL_GRACE_NS UINT64_C(2000000000)
#define REAP_GRACE_NS UINT64_C(500000000)
#define SCHEMA "moriarty.prover-lifetime/1"

struct control_record {
    char boot_id[37];
    uint64_t time_namespace_inode;
    uint64_t latest_start_ns;
    uint64_t kill_deadline_ns;
    char invocation_digest[65];
};

static volatile sig_atomic_t caught_signal;
static volatile sig_atomic_t caught_child;

static void usage(void)
{
    fputs("usage: prover_lifetime <control-file-path> -- <proof-server-path> [proof-server-arg...]\n",
          stderr);
}

static int refuse(const char *reason)
{
    fprintf(stderr, "refuse: %s\n", reason);
    return 64;
}

static int is_lower_hex(char ch)
{
    return (ch >= '0' && ch <= '9') || (ch >= 'a' && ch <= 'f');
}

static int valid_uuid(const char *value)
{
    size_t i;

    if (strlen(value) != 36U) {
        return 0;
    }
    for (i = 0; i < 36U; ++i) {
        if (i == 8U || i == 13U || i == 18U || i == 23U) {
            if (value[i] != '-') {
                return 0;
            }
        } else if (!is_lower_hex(value[i])) {
            return 0;
        }
    }
    return 1;
}

static int valid_digest(const char *value)
{
    size_t i;

    if (strlen(value) != 64U) {
        return 0;
    }
    for (i = 0; i < 64U; ++i) {
        if (!is_lower_hex(value[i])) {
            return 0;
        }
    }
    return 1;
}

static int parse_u64(const char *value, uint64_t *result)
{
    uint64_t parsed = 0;
    const unsigned char *cursor = (const unsigned char *)value;

    if (*cursor == '\0' || (*cursor == '0' && cursor[1] != '\0')) {
        return 0;
    }
    while (*cursor != '\0') {
        unsigned int digit;
        if (*cursor < '0' || *cursor > '9') {
            return 0;
        }
        digit = (unsigned int)(*cursor - '0');
        if (parsed > (UINT64_MAX - digit) / UINT64_C(10)) {
            return 0;
        }
        parsed = parsed * UINT64_C(10) + digit;
        ++cursor;
    }
    *result = parsed;
    return 1;
}

static int parse_record(unsigned char *bytes, size_t length,
                        struct control_record *record)
{
    char *lines[6];
    size_t line_count = 0;
    size_t i;

    if (length > CONTROL_MAX_BYTES || length == 0U) {
        return 0;
    }
    lines[0] = (char *)bytes;
    for (i = 0; i < length; ++i) {
        if (bytes[i] == '\r' || bytes[i] == '\0' || bytes[i] > 0x7fU) {
            return 0;
        }
        if (bytes[i] == '\n') {
            if (line_count >= 6U) {
                return 0;
            }
            bytes[i] = '\0';
            ++line_count;
            if (line_count < 6U) {
                lines[line_count] = (char *)&bytes[i + 1U];
            } else if (i + 1U != length) {
                return 0;
            }
        }
    }
    if (line_count != 6U || bytes[length - 1U] != '\0') {
        return 0;
    }
    if (strcmp(lines[0], SCHEMA) != 0 || !valid_uuid(lines[1]) ||
        !parse_u64(lines[2], &record->time_namespace_inode) ||
        !parse_u64(lines[3], &record->latest_start_ns) ||
        !parse_u64(lines[4], &record->kill_deadline_ns) ||
        !valid_digest(lines[5])) {
        return 0;
    }
    memcpy(record->boot_id, lines[1], sizeof(record->boot_id));
    memcpy(record->invocation_digest, lines[5], sizeof(record->invocation_digest));
    return 1;
}

static int read_bounded_fd(int fd, unsigned char *bytes, size_t *length)
{
    size_t used = 0;

    while (used < CONTROL_MAX_BYTES + 1U) {
        ssize_t amount = read(fd, bytes + used, CONTROL_MAX_BYTES + 1U - used);
        if (amount > 0) {
            used += (size_t)amount;
        } else if (amount == 0) {
            *length = used;
            return 1;
        } else if (errno != EINTR) {
            return 0;
        }
    }
    *length = used;
    return 1;
}

static int read_control(const char *path, unsigned char *bytes, size_t *length,
                        const char **reason)
{
    int fd = open(path, O_RDONLY | O_NOFOLLOW | O_CLOEXEC | O_NONBLOCK);
    struct stat details;
    int saved_errno;

    if (fd < 0) {
        *reason = "control open";
        return 0;
    }
    if (fstat(fd, &details) != 0) {
        *reason = "control stat";
        close(fd);
        return 0;
    }
    if (!S_ISREG(details.st_mode)) {
        *reason = "control nonregular";
        close(fd);
        return 0;
    }
    if (!read_bounded_fd(fd, bytes, length)) {
        saved_errno = errno;
        close(fd);
        errno = saved_errno;
        *reason = "control read";
        return 0;
    }
    if (close(fd) != 0) {
        *reason = "control close";
        return 0;
    }
    if (*length > CONTROL_MAX_BYTES) {
        *reason = "control oversize";
        return 0;
    }
    return 1;
}

static int monotonic_ns(uint64_t *result)
{
    struct timespec now;
    uint64_t seconds;

    if (clock_gettime(CLOCK_MONOTONIC, &now) != 0 || now.tv_sec < 0 ||
        now.tv_nsec < 0 || now.tv_nsec >= 1000000000L) {
        return 0;
    }
    seconds = (uint64_t)now.tv_sec;
    if (seconds > (UINT64_MAX - (uint64_t)now.tv_nsec) / UINT64_C(1000000000)) {
        return 0;
    }
    *result = seconds * UINT64_C(1000000000) + (uint64_t)now.tv_nsec;
    return 1;
}

static uint64_t add_saturating(uint64_t value, uint64_t addend)
{
    return value > UINT64_MAX - addend ? UINT64_MAX : value + addend;
}

static int read_boot_id(char result[37])
{
    unsigned char bytes[64];
    size_t used = 0;
    ssize_t amount = 0;
    unsigned char extra;
    int fd = open("/proc/sys/kernel/random/boot_id", O_RDONLY | O_CLOEXEC);

    if (fd < 0) {
        return 0;
    }
    while (used < sizeof(bytes)) {
        do {
            amount = read(fd, bytes + used, sizeof(bytes) - used);
        } while (amount < 0 && errno == EINTR);
        if (amount < 0) {
            close(fd);
            return 0;
        }
        if (amount == 0) {
            break;
        }
        used += (size_t)amount;
    }
    if (used == sizeof(bytes)) {
        do {
            amount = read(fd, &extra, 1U);
        } while (amount < 0 && errno == EINTR);
        if (amount != 0) {
            close(fd);
            return 0;
        }
    }
    if (close(fd) != 0) {
        return 0;
    }
    if (used == 37U && bytes[36] == '\n') {
        used = 36U;
    }
    if (used != 36U) {
        return 0;
    }
    memcpy(result, bytes, 36U);
    result[36] = '\0';
    return valid_uuid(result);
}

static int clock_identity_matches(const struct control_record *record,
                                  const char **reason)
{
    char boot_id[37];
    struct stat time_namespace;

    if (!read_boot_id(boot_id)) {
        *reason = "boot id read";
        return 0;
    }
    if (strcmp(boot_id, record->boot_id) != 0) {
        *reason = "boot id mismatch";
        return 0;
    }
    if (stat("/proc/self/ns/time", &time_namespace) != 0) {
        *reason = "time namespace read";
        return 0;
    }
    if ((uintmax_t)time_namespace.st_ino > UINT64_MAX ||
        (uint64_t)time_namespace.st_ino != record->time_namespace_inode) {
        *reason = "time namespace mismatch";
        return 0;
    }
    return 1;
}

static void signal_handler(int number)
{
    if (number == SIGCHLD) {
        caught_child = 1;
    } else {
        caught_signal = number;
    }
}

static int install_handlers(sigset_t *blocked, sigset_t *wait_mask)
{
    struct sigaction action;
    const int signals[] = {SIGTERM, SIGINT, SIGQUIT, SIGHUP, SIGCHLD};
    size_t i;

    sigemptyset(blocked);
    for (i = 0; i < sizeof(signals) / sizeof(signals[0]); ++i) {
        sigaddset(blocked, signals[i]);
    }
    if (sigprocmask(SIG_BLOCK, blocked, wait_mask) != 0) {
        return 0;
    }
    for (i = 0; i < sizeof(signals) / sizeof(signals[0]); ++i) {
        sigdelset(wait_mask, signals[i]);
    }
    memset(&action, 0, sizeof(action));
    action.sa_handler = signal_handler;
    sigemptyset(&action.sa_mask);
    for (i = 0; i < sizeof(signals) / sizeof(signals[0]); ++i) {
        if (sigaction(signals[i], &action, NULL) != 0) {
            return 0;
        }
    }
    return 1;
}

static void reset_child_signals(const sigset_t *wait_mask)
{
    struct sigaction action;
    const int signals[] = {SIGTERM, SIGINT, SIGQUIT, SIGHUP, SIGCHLD};
    size_t i;

    memset(&action, 0, sizeof(action));
    action.sa_handler = SIG_DFL;
    sigemptyset(&action.sa_mask);
    for (i = 0; i < sizeof(signals) / sizeof(signals[0]); ++i) {
        (void)sigaction(signals[i], &action, NULL);
    }
    (void)sigprocmask(SIG_SETMASK, wait_mask, NULL);
}

static struct timespec ns_interval(uint64_t from, uint64_t until)
{
    uint64_t remaining = until > from ? until - from : 0;
    struct timespec interval;

    interval.tv_sec = (time_t)(remaining / UINT64_C(1000000000));
    interval.tv_nsec = (long)(remaining % UINT64_C(1000000000));
    return interval;
}

static int reap_tracked(pid_t child, int *status)
{
    pid_t result;

    do {
        result = waitpid(child, status, WNOHANG);
    } while (result < 0 && errno == EINTR);
    return result == child;
}

static void final_reap(pid_t child_pgid, uint64_t deadline)
{
    uint64_t now;
    uint64_t reap_end;
    sigset_t empty;

    (void)kill(-child_pgid, SIGKILL);
    if (!monotonic_ns(&now) || now >= deadline) {
        while (waitpid(-1, NULL, WNOHANG) > 0) {
        }
        return;
    }
    reap_end = add_saturating(now, REAP_GRACE_NS);
    if (reap_end > deadline) {
        reap_end = deadline;
    }
    sigemptyset(&empty);
    for (;;) {
        pid_t result;
        int saw_child = 0;
        do {
            result = waitpid(-1, NULL, WNOHANG);
            if (result > 0) {
                saw_child = 1;
            }
        } while (result > 0 || (result < 0 && errno == EINTR));
        if (result < 0 && errno == ECHILD) {
            return;
        }
        if (!monotonic_ns(&now) || now >= reap_end) {
            return;
        }
        if (!saw_child) {
            struct timespec interval = ns_interval(now, reap_end);
            (void)ppoll(NULL, 0, &interval, &empty);
        }
    }
}

static int supervise(pid_t child, uint64_t deadline, const sigset_t *wait_mask)
{
    int tracked_status = 0;
    int tracked_done = 0;
    int timed_out = 0;
    int forwarded = 0;
    int forwarded_signal = 0;
    uint64_t signal_end = 0;

    for (;;) {
        uint64_t now;
        uint64_t wait_until;
        struct timespec interval;

        if (caught_child) {
            caught_child = 0;
        }
        if (reap_tracked(child, &tracked_status)) {
            tracked_done = 1;
            break;
        }
        if (!monotonic_ns(&now)) {
            timed_out = 1;
            (void)kill(-child, SIGKILL);
            break;
        }
        if (now >= deadline) {
            timed_out = 1;
            (void)kill(-child, SIGKILL);
            break;
        }
        if (caught_signal != 0) {
            int number = caught_signal;
            caught_signal = 0;
            (void)kill(-child, number);
            if (!forwarded) {
                forwarded_signal = number;
                signal_end = add_saturating(now, SIGNAL_GRACE_NS);
                if (signal_end > deadline) {
                    signal_end = deadline;
                }
                forwarded = 1;
            }
        }
        if (forwarded && now >= signal_end) {
            (void)kill(-child, SIGKILL);
        }
        wait_until = deadline;
        if (forwarded && signal_end < wait_until) {
            wait_until = signal_end;
        }
        interval = ns_interval(now, wait_until);
        (void)ppoll(NULL, 0, &interval, wait_mask);
    }

    final_reap(child, deadline);
    if (timed_out) {
        return 124;
    }
    if (!tracked_done) {
        return 125;
    }
    if (WIFEXITED(tracked_status)) {
        if (forwarded) {
            return 128 + forwarded_signal;
        }
        return WEXITSTATUS(tracked_status);
    }
    if (WIFSIGNALED(tracked_status)) {
        return 128 + WTERMSIG(tracked_status);
    }
    return 125;
}

static int selftest_encode(int argc, char **argv)
{
    struct control_record record;

    if (argc != 7 || !valid_uuid(argv[2]) ||
        !parse_u64(argv[3], &record.time_namespace_inode) ||
        !parse_u64(argv[4], &record.latest_start_ns) ||
        !parse_u64(argv[5], &record.kill_deadline_ns) ||
        !valid_digest(argv[6])) {
        fputs("selftest encode input rejected\n", stderr);
        return 2;
    }
    if (printf(SCHEMA "\n%s\n%" PRIu64 "\n%" PRIu64 "\n%" PRIu64 "\n%s\n",
               argv[2], record.time_namespace_inode, record.latest_start_ns,
               record.kill_deadline_ns, argv[6]) < 0) {
        return 1;
    }
    return 0;
}

static int selftest_decode(int argc)
{
    unsigned char bytes[CONTROL_MAX_BYTES + 1U];
    size_t length;
    struct control_record record;

    if (argc != 2) {
        fputs("selftest decode input rejected\n", stderr);
        return 2;
    }
    if (!read_bounded_fd(STDIN_FILENO, bytes, &length) ||
        length > CONTROL_MAX_BYTES || !parse_record(bytes, length, &record)) {
        fputs("selftest decode input rejected\n", stderr);
        return 2;
    }
    if (printf("%s\n%" PRIu64 "\n%" PRIu64 "\n%" PRIu64 "\n%s\n",
               record.boot_id, record.time_namespace_inode,
               record.latest_start_ns, record.kill_deadline_ns,
               record.invocation_digest) < 0) {
        return 1;
    }
    return 0;
}

int main(int argc, char **argv)
{
    uint64_t entry_ns;
    uint64_t own_deadline;
    unsigned char bytes[CONTROL_MAX_BYTES + 1U];
    size_t length;
    struct control_record record;
    const char *reason;
    sigset_t blocked;
    sigset_t wait_mask;
    pid_t child;

    if (argc >= 2 && strcmp(argv[1], "--selftest-encode") == 0) {
        return selftest_encode(argc, argv);
    }
    if (argc >= 2 && strcmp(argv[1], "--selftest-decode") == 0) {
        return selftest_decode(argc);
    }
    if (!monotonic_ns(&entry_ns)) {
        return refuse("monotonic clock");
    }
    if (argc < 4 || strcmp(argv[2], "--") != 0) {
        usage();
        return 2;
    }
    if (!read_control(argv[1], bytes, &length, &reason)) {
        return refuse(reason);
    }
    if (!parse_record(bytes, length, &record)) {
        return refuse("control malformed");
    }
    if (!clock_identity_matches(&record, &reason)) {
        return refuse(reason);
    }
    if (entry_ns >= record.latest_start_ns) {
        return refuse("late entry");
    }
    if (entry_ns >= record.kill_deadline_ns) {
        return refuse("expired deadline");
    }
    own_deadline = add_saturating(entry_ns, OWN_RUNTIME_NS);
    if (!(own_deadline < record.kill_deadline_ns)) {
        own_deadline = record.kill_deadline_ns;
    }
    if (!install_handlers(&blocked, &wait_mask)) {
        return refuse("signal setup");
    }
    (void)prctl(PR_SET_CHILD_SUBREAPER, 1, 0, 0, 0);
    child = fork();
    if (child < 0) {
        return refuse("fork");
    }
    if (child == 0) {
        if (setpgid(0, 0) != 0) {
            _exit(126);
        }
        reset_child_signals(&wait_mask);
        execv(argv[3], &argv[3]);
        _exit(126);
    }
    if (setpgid(child, child) != 0 && errno != EACCES && errno != ESRCH) {
        (void)kill(child, SIGKILL);
    }
    return supervise(child, own_deadline, &wait_mask);
}
