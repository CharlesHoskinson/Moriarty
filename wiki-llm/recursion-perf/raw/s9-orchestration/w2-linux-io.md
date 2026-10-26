# Linux process I/O counter semantics
> Source: https://man7.org/linux/man-pages/man5/proc_pid_io.5.html
> Collected: 2026-10-26
> Published: Unknown

proc\_pid\_io(5) - Linux manual page

|  |  |
| --- | --- |
| [man7.org](../../../index.html) > Linux > [man-pages](../index.html) | [Linux/UNIX system programming training](http://man7.org/training/) |

---

proc\_pid\_io(5) — Linux manual page
====================================

|  |
| --- |
| [NAME](#NAME) | [DESCRIPTION](#DESCRIPTION) | [CAVEATS](#CAVEATS) | [SEE ALSO](#SEE_ALSO) | [COLOPHON](#COLOPHON) |
|  |  |

```
proc_pid_io(5)             File Formats Manual             proc_pid_io(5)
```

NAME         [top](#top_of_page)
--------------------------------

```
       /proc/pid/io - I/O statistics
```

DESCRIPTION         [top](#top_of_page)
---------------------------------------

```
       /proc/pid/io (since Linux 2.6.20)
              This file contains I/O statistics for the process and its
              waited-for children, for example:

                  # cat /proc/3828/io
                  rchar: 323934931
                  wchar: 323929600
                  syscr: 632687
                  syscw: 632675
                  read_bytes: 0
                  write_bytes: 323932160
                  cancelled_write_bytes: 0

              The fields are as follows:

              rchar: characters read
                     The number of bytes returned by successful read(2)
                     and similar system calls.

              wchar: characters written
                     The number of bytes returned by successful write(2)
                     and similar system calls.

              syscr: read syscalls
                     The number of "file read" system calls—those from
                     the read(2) family, sendfile(2), copy_file_range(2),
                     and ioctl(2) BTRFS_IOC_ENCODED_READ[_32] (including
                     when invoked by the kernel as part of other
                     syscalls).

              syscw: write syscalls
                     The number of "file write" system calls—those from
                     the write(2) family, sendfile(2),
                     copy_file_range(2), and ioctl(2)
                     BTRFS_IOC_ENCODED_WRITE[_32] (including when invoked
                     by the kernel as part of other syscalls).

              read_bytes: bytes read
                     The number of bytes really fetched from the storage
                     layer.  This is accurate for block-backed
                     filesystems.

              write_bytes: bytes written
                     The number of bytes really sent to the storage
                     layer.

              cancelled_write_bytes:
                     The above statistics fail to account for truncation:
                     if a process writes 1 MB to a regular file and then
                     removes it, said 1 MB will not be written, but will
                     have nevertheless been accounted as a 1 MB write.
                     This field represents the number of bytes "saved"
                     from I/O writeback.  This can yield to having done
                     negative I/O if caches dirtied by another process
                     are truncated.  cancelled_write_bytes applies to I/O
                     already accounted-for in write_bytes.

              Permission to access this file is governed by ptrace(2)
              access mode PTRACE_MODE_READ_FSCREDS.
```

CAVEATS         [top](#top_of_page)
-----------------------------------

```
       These counters are not atomic: on systems where 64-bit integer
       operations may tear, a counter could be updated simultaneously
       with a read, yielding an incorrect intermediate value.
```

SEE ALSO         [top](#top_of_page)
------------------------------------

```
       getrusage(2), proc(5)
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

Linux man-pages 6.19            2026-02-08                 proc_pid_io(5)
```

---

Pages that refer to this page:
[getrusage(2)](../man2/getrusage.2.html), 
[proc\_pid(5)](../man5/proc_pid.5.html)

---

---

|  |  |  |
| --- | --- | --- |
| HTML rendering created 2026-09-09 by [Michael Kerrisk](https://man7.org/mtk/index.html), author of [*The Linux Programming Interface*](https://man7.org/tlpi/).  For details of in-depth **Linux/UNIX system programming training courses** that I teach, look [here](https://man7.org/training/).  Hosting by [jambit GmbH](https://www.jambit.com/index_en.html). |  | [Cover of TLPI](https://man7.org/tlpi/) |

---

[![Web Analytics Made Easy -
StatCounter](https://c.statcounter.com/7422636/0/9b6714ff/1/)](https://statcounter.com/ "Web Analytics
Made Easy - StatCounter")