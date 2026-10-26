# getrusage(2) - Linux manual page
> Source: https://man7.org/linux/man-pages/man2/getrusage.2.html
> Collected: 2026-10-26
> Published: 2026-02-08

getrusage(2) - Linux manual page

|  |  |
| --- | --- |
| [man7.org](../../../index.html) > Linux > [man-pages](../index.html) | [Linux/UNIX system programming training](http://man7.org/training/) |

---

getrusage(2) — Linux manual page
================================

|  |
| --- |
| [NAME](#NAME) | [LIBRARY](#LIBRARY) | [SYNOPSIS](#SYNOPSIS) | [DESCRIPTION](#DESCRIPTION) | [RETURN VALUE](#RETURN_VALUE) | [ERRORS](#ERRORS) | [ATTRIBUTES](#ATTRIBUTES) | [STANDARDS](#STANDARDS) | [HISTORY](#HISTORY) | [NOTES](#NOTES) | [SEE ALSO](#SEE_ALSO) | [COLOPHON](#COLOPHON) |
|  |  |

```
getrusage(2)               System Calls Manual               getrusage(2)
```

NAME         [top](#top_of_page)
--------------------------------

```
       getrusage - get resource usage
```

LIBRARY         [top](#top_of_page)
-----------------------------------

```
       Standard C library (libc, -lc)
```

SYNOPSIS         [top](#top_of_page)
------------------------------------

```
       #include <sys/resource.h>

       int getrusage(int who, struct rusage *usage);
```

DESCRIPTION         [top](#top_of_page)
---------------------------------------

```
       getrusage() returns resource usage measures for who, which can be
       one of the following:

       RUSAGE_SELF
              Return resource usage statistics for the calling process,
              which is the sum of resources used by all threads in the
              process.

       RUSAGE_CHILDREN
              Return resource usage statistics for all children of the
              calling process that have terminated and been waited for.
              These statistics will include the resources used by
              grandchildren, and further removed descendants, if all of
              the intervening descendants waited on their terminated
              children.

       RUSAGE_THREAD (since Linux 2.6.26)
              Return resource usage statistics for the calling thread.
              The _GNU_SOURCE feature test macro must be defined (before
              including any header file) in order to obtain the
              definition of this constant from <sys/resource.h>.

       The resource usages are returned in the structure pointed to by
       usage, which has the following form:

           struct rusage {
               struct timeval ru_utime; /* user CPU time used */
               struct timeval ru_stime; /* system CPU time used */
               long   ru_maxrss;        /* maximum resident set size */
               long   ru_ixrss;         /* integral shared memory size */
               long   ru_idrss;         /* integral unshared data size */
               long   ru_isrss;         /* integral unshared stack size */
               long   ru_minflt;        /* page reclaims (soft page faults) */
               long   ru_majflt;        /* page faults (hard page faults) */
               long   ru_nswap;         /* swaps */
               long   ru_inblock;       /* block input operations */
               long   ru_oublock;       /* block output operations */
               long   ru_msgsnd;        /* IPC messages sent */
               long   ru_msgrcv;        /* IPC messages received */
               long   ru_nsignals;      /* signals received */
               long   ru_nvcsw;         /* voluntary context switches */
               long   ru_nivcsw;        /* involuntary context switches */
           };

       Not all fields are completed; unmaintained fields are set to zero
       by the kernel.  (The unmaintained fields are provided for
       compatibility with other systems, and because they may one day be
       supported on Linux.)  The fields are interpreted as follows:

       ru_utime
              This is the total amount of time spent executing in user
              mode, expressed in a timeval structure (seconds plus
              microseconds).

       ru_stime
              This is the total amount of time spent executing in kernel
              mode, expressed in a timeval structure (seconds plus
              microseconds).

       ru_maxrss (since Linux 2.6.32)
              This is the maximum resident set size used (in KiB).  For
              RUSAGE_CHILDREN, this is the resident set size of the
              largest child, not the maximum resident set size of the
              process tree.

       ru_ixrss (unmaintained)
              This field is currently unused on Linux.

       ru_idrss (unmaintained)
              This field is currently unused on Linux.

       ru_isrss (unmaintained)
              This field is currently unused on Linux.

       ru_minflt
              The number of page faults serviced without any I/O
              activity; here, I/O activity is avoided by “reclaiming” a
              page frame from the list of pages awaiting reallocation.

       ru_majflt
              The number of page faults serviced that required I/O
              activity.

       ru_nswap (unmaintained)
              This field is currently unused on Linux.

       ru_inblock (since Linux 2.6.22)
              The number of times the filesystem had to perform input.

       ru_oublock (since Linux 2.6.22)
              The number of times the filesystem had to perform output.

       ru_msgsnd (unmaintained)
              This field is currently unused on Linux.

       ru_msgrcv (unmaintained)
              This field is currently unused on Linux.

       ru_nsignals (unmaintained)
              This field is currently unused on Linux.

       ru_nvcsw (since Linux 2.6)
              The number of times a context switch resulted due to a
              process voluntarily giving up the processor before its time
              slice was completed (usually to await availability of a
              resource).

       ru_nivcsw (since Linux 2.6)
              The number of times a context switch resulted due to a
              higher priority process becoming runnable or because the
              current process exceeded its time slice.
```

RETURN VALUE         [top](#top_of_page)
----------------------------------------

```
       On success, zero is returned.  On error, -1 is returned, and errno
       is set to indicate the error.
```

ERRORS         [top](#top_of_page)
----------------------------------

```
       EFAULT usage points outside the accessible address space.

       EINVAL who is invalid.
```

ATTRIBUTES         [top](#top_of_page)
--------------------------------------

```
       For an explanation of the terms used in this section, see
       attributes(7).
       ┌──────────────────────────────────────┬───────────────┬─────────┐
       │ Interface                            │ Attribute     │ Value   │
       ├──────────────────────────────────────┼───────────────┼─────────┤
       │ getrusage()                          │ Thread safety │ MT-Safe │
       └──────────────────────────────────────┴───────────────┴─────────┘
```

STANDARDS         [top](#top_of_page)
-------------------------------------

```
       POSIX.1-2024.

       POSIX.1 specifies getrusage(), but specifies only the fields
       ru_utime and ru_stime.

       RUSAGE_THREAD is Linux-specific.
```

HISTORY         [top](#top_of_page)
-----------------------------------

```
       4.3BSD, SVr4, SUSv1, POSIX.1-2001 XSI.

       Before Linux 2.6.9, if the disposition of SIGCHLD is set to
       SIG_IGN then the resource usages of child processes are
       automatically included in the value returned by RUSAGE_CHILDREN,
       although POSIX.1-2001 explicitly prohibits this.  This
       nonconformance is rectified in Linux 2.6.9 and later.

       The structure definition shown at the start of this page was taken
       from 4.3BSD Reno.

       Ancient systems provided a vtimes() function with a similar
       purpose to getrusage().  For backward compatibility, glibc (up
       until Linux 2.32) also provides vtimes().  All new applications
       should be written using getrusage().  (Since Linux 2.33, glibc no
       longer provides an vtimes() implementation.)
```

NOTES         [top](#top_of_page)
---------------------------------

```
       Resource usage metrics are preserved across an execve(2).
```

SEE ALSO         [top](#top_of_page)
------------------------------------

```
       clock_gettime(2), getrlimit(2), times(2), wait(2), wait4(2),
       clock(3), proc_pid_stat(5), proc_pid_io(5)
```

COLOPHON         [top](#top_of_page)
------------------------------------

```
       This page is part of the man-pages (Linux kernel and C library
       user-space interface documentation) project.  Information about
       the project can be found at 
       ⟨https://www.kernel.org/doc/man-pages/⟩.  If you have a bug report
       for this manual page, see
       ⟨https://git.kernel.org/pub/scm/docs/man-pages/man-pages.git/tree/CONTRIBUTING⟩.
       This page was obtained from the tarball man-pages-6.19.tar.gz
       fetched from
       ⟨https://mirrors.edge.kernel.org/pub/linux/docs/man-pages/⟩ on
       2026-09-09.  If you discover any rendering problems in this HTML
       version of the page, or you believe there is a better or more up-
       to-date source for the page, or you have corrections or
       improvements to the information in this COLOPHON (which is not
       part of the original manual page), send a mail to
       man-pages@man7.org

Linux man-pages 6.19            2026-02-08                   getrusage(2)
```

---

Pages that refer to this page:
[fork(2)](../man2/fork.2.html), 
[getrlimit(2)](../man2/getrlimit.2.html), 
[sigaction(2)](../man2/sigaction.2.html), 
[syscalls(2)](../man2/syscalls.2.html), 
[times(2)](../man2/times.2.html), 
[wait(2)](../man2/wait.2.html), 
[wait4(2)](../man2/wait4.2.html), 
[clock(3)](../man3/clock.3.html), 
[pmwebtimerregister(3)](../man3/pmwebtimerregister.3.html), 
[proc\_pid\_io(5)](../man5/proc_pid_io.5.html), 
[pthreads(7)](../man7/pthreads.7.html), 
[time(7)](../man7/time.7.html)

---

---

|  |  |  |
| --- | --- | --- |
| HTML rendering created 2026-09-09 by [Michael Kerrisk](https://man7.org/mtk/index.html), author of [*The Linux Programming Interface*](https://man7.org/tlpi/).  For details of in-depth **Linux/UNIX system programming training courses** that I teach, look [here](https://man7.org/training/).  Hosting by [jambit GmbH](https://www.jambit.com/index_en.html). |  | [Cover of TLPI](https://man7.org/tlpi/) |

---

[![Web Analytics Made Easy -
StatCounter](https://c.statcounter.com/7422636/0/9b6714ff/1/)](https://statcounter.com/ "Web Analytics
Made Easy - StatCounter")
