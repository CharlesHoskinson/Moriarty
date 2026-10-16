# K5 operator: capability probe body for step Q5 (view `sbx_run`, clause K5-2). PROPOSED SOURCE; never executed by its author.
# Runs only INSIDE sbx_run, as:
#   bash --noprofile --norc -c "<this text>" probe-k5 <host T5> <host user ns> <host pid ns> <host net ns> <host ipc ns> <host uts ns>
# The text arrives in argv because sbx_run mounts no probe path: the view holds /task/{bin,in,out,keys} only.
# It creates, writes and deletes nothing. Writability is read from the mount table and access(2) (`test -w`), never
# by creating a marker, so /task/out and /task/keys stay empty for K5M. It runs no mori-k2 command, draws no
# randomness and never names, stats, lists or reads a host credential, key, cache or private path. It avoids awk
# and other /etc/alternatives links, which the view does not mount.
# Gating checks call fail(); lines marked "record" decide nothing.
set -u
fail() { echo "PROBE-FAIL: $*"; exit 1; }
test "$#" -eq 6 || fail "usage: probe-k5 <host T5> <user> <pid> <net> <ipc> <uts>"
HOSTROOT="$1"
shift
echo "probe-start view=sbx_run"
echo "record uid=$(id -u) gid=$(id -g)"
echo "record status:"; grep -E '^(Uid|Gid|CapInh|CapPrm|CapEff|CapBnd|CapAmb|NoNewPrivs|Seccomp):' /proc/self/status
CE="$(grep -E '^CapEff:' /proc/self/status)" || fail "no CapEff line"
test "${CE##*[[:space:]]}" = 0000000000000000 || fail "effective capabilities present"
# 1 namespaces: each differs from the namespace of the host step that launched the view
for n in user pid net ipc uts; do
  IN="$(readlink "/proc/self/ns/$n")" || fail "cannot read /proc/self/ns/$n"
  echo "record ns $n view=$IN host=$1"
  { test -n "$1" && test "$IN" != "$1"; } || fail "$n namespace is the host's"
  shift
done
# 2 network: loopback only
cat /proc/net/dev
mapfile -t ND < /proc/net/dev
test "${#ND[@]}" -eq 3 || fail "network interfaces other than lo: $(( ${#ND[@]} - 2 )) listed"
IF="${ND[2]%%:*}"
IF="${IF//[[:space:]]/}"
test "$IF" = lo || fail "the only interface is $IF, not lo"
# 3 pid namespace: only sandbox processes are visible
n=0
for p in /proc/[0-9]*; do n=$((n + 1)); done
echo "record visible_pids=$n"
test "$n" -le 16 || fail "pid namespace not isolated"
# 4 no host task root, host home, toolchain, registry, target or build tree (positive control: Q5 tests T5/logs on the host)
test ! -e "$HOSTROOT/logs" || fail "host task logs visible"
test ! -e "$HOSTROOT" || fail "host task root visible"
for p in /home /root /nonexistent /etc/alternatives /task/k2 /task/r1cs /task/moriarty /task/probe /task/target /task/cargo-home /task/home /task/tmp /task/gate; do
  test ! -e "$p" || fail "path visible in the view: $p"
done
for b in cargo rustc rustup; do
  if command -v "$b" > /dev/null 2>&1; then echo "record: $b resolves to $(command -v "$b") under the read-only /usr or /bin; no task toolchain is mounted"; fi
done
# 5 view shape
ROOTL="$(ls -A / | LC_ALL=C sort | tr '\n' ' ')"
echo "record root_entries=$ROOTL"
case "$ROOTL" in
  "bin dev etc lib lib64 proc task tmp usr "|"bin dev lib lib64 proc task tmp usr ") ;;
  *) fail "unexpected entries at the view root: $ROOTL" ;;
esac
if test -e /etc; then test "$(ls -A /etc | tr '\n' ' ')" = "ld.so.cache " || fail "unexpected /etc entries"; fi
test "$(ls -A /task | LC_ALL=C sort | tr '\n' ' ')" = "bin in keys out " || fail "unexpected /task entries"
test "$(ls -A /task/bin | tr '\n' ' ')" = "mori-k2 " || fail "unexpected /task/bin entries"
test "$(ls -A /task/in | LC_ALL=C sort | tr '\n' ' ')" = "conditional-reserve.mori installment-invoice.mori invoice-partial.selector.json reserve-allocate.selector.json " || fail "unexpected /task/in entries"
X="$(find /task/out /task/keys -mindepth 1 -print -quit)" || fail "find could not list /task/out and /task/keys"
test -z "$X" || fail "pre-existing entry $X"
test "$(stat -c %a /task/keys)" = 700 || fail "/task/keys is not mode 0700"
{ test -f /task/bin/mori-k2 && test -x /task/bin/mori-k2 && test ! -u /task/bin/mori-k2 && test ! -g /task/bin/mori-k2; } || fail "mori-k2 is not a plain executable file"
# 6 read-only and read-write mounts: the per-mount options of the last mount at each point, then access(2)
mnt() { local want="$1" id par dev root mp rest hit=""; while read -r id par dev root mp rest; do test "$mp" = "$want" && hit="$rest"; done < /proc/self/mountinfo; printf '%s' "$hit"; }
for d in /usr /bin /lib /lib64 /task/bin /task/in; do
  R="$(mnt "$d")"; O="${R%% *}"
  echo "record mount $d options=$O"
  case ",$O," in *,ro,*) ;; *) fail "$d is not mounted read-only" ;; esac
  test ! -w "$d" || fail "$d is writable"
done
for d in /task/out /task/keys; do
  R="$(mnt "$d")"; O="${R%% *}"
  echo "record mount $d options=$O"
  case ",$O," in *,rw,*) ;; *) fail "$d is not mounted read-write" ;; esac
  test -w "$d" || fail "$d is not writable"
done
# 7 tmpfs /tmp, synthetic /dev and stdin. The random devices are only tested for existence, never read.
R="$(mnt /tmp)"; FT="${R#* - }"
echo "record mount /tmp $R"
test "${FT%% *}" = tmpfs || fail "/tmp is not tmpfs"
{ test -c /dev/null && test -c /dev/urandom && test -c /dev/random; } || fail "synthetic /dev lacks null, urandom or random"
test "$(readlink /proc/self/fd/0)" = /dev/null || fail "stdin is not /dev/null"
# 8 environment
echo "record env:"; env | LC_ALL=C sort
{ test "$HOME" = /nonexistent && test "$PATH" = /usr/bin:/bin && test "$PWD" = /task; } || fail "HOME, PATH or the working directory differs"
for v in $(compgen -e); do
  case "$v" in HOME|PATH|PWD|SHLVL|OLDPWD|_) ;; *) fail "unexpected environment variable $v" ;; esac
done
# 9 GNU time works inside the view (clause K-6 measurement, reused by K5-4M)
test -x /usr/bin/time || fail "/usr/bin/time absent"
/usr/bin/time -v /usr/bin/true 2>&1 | grep -q "Maximum resident set size" || fail "GNU time -v gives no maximum resident set size"
# 10 limits inherited from the step wrapper (record)
cat /proc/self/limits
echo "probe-ok view=sbx_run"
