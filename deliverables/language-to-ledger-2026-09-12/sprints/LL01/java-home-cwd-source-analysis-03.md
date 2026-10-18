The inspected OpenJDK source supports `user.home=/home/charl` structurally under real UID1000 and the current files-first NSS configuration. It does not establish the installed Java runtime observation. No new vote or native admission.

**primary-source fact**. OpenJDK jdk-21+35 java_props_md.c lines481-500 uses getpwuid(getuid()) on Linux, taking pw_dir as user_home. If missing or shorter than two characters it falls back to nonempty HOME, otherwise question-mark. This uses real UID, so validating effective UID alone is insufficient for the source argument.

**primary-source fact**. SystemProps.java lines68-75 starts with VM/command properties and adds user.home only when absent. A -Duser.home override can take precedence; original fixed loader JVM options contain none, and injected Java option environments remain excluded.

**repository observation**. The sole /etc/passwd UID1000 entry is charl:x:1000:1000::/home/charl:/bin/bash. nsswitch passwd routing is files systemd, with no action override. Local nsswitch manual says successful lookup defaults to return, so a successful files lookup does not consult systemd. These files and /etc are host-root owned, not writable by the current user.

**inference**. With real UID1000, readable preserved NSS/passwd, no property override and the installed JVM following the inspected upstream mechanism, user.home resolves to /home/charl. Private mount covering that same absolute path therefore covers Java home reads. This is structural support, not actual Java execution. Missing files/read errors, altered NSS or overrides invalidate the argument.

**repository observation**. Selected JRE release metadata states JAVA_VERSION=21 and the minimal module set; store pathname identifies21+35. No matching local source archive was found; the discovered21.0.9+10 archive is another build and was not substituted. Static ELF inspection of selected libjava.so shows getpwuid/getuid/getcwd imports and RUNPATH to glibc2.39-52. Symbols support but do not prove source/build correspondence or runtime NSS selection.

**source fact**. Original expression-parser reads its adjacent lock then execs kast; unwrapped kast invokes k -kast. k changes directory only inside command substitution to locate trusted K metadata. Shell path has no intended original-cwd file write. Both input and output Java inherit the same namespace and original HOME.

**source fact**. k-util.sh uses mktemp default parent (with fixed environment and no temp-dir argument, /tmp), creates per-tool temporary directories, and removes those through a trap. krun redirects parser stdout and creates input/expanded/result files in this private temp tree. It invokes llvm-krun in a subshell whose cwd is tempDir. llvm-krun starts with four relative mktemp files and later writes assembled input there; retained --dry-run -nm -o basename path writes its assembled result there rather than launching an additional interpreter.

**source fact**. The selected krun LLVM interpreter receives explicit expanded-input and result.kore paths in tempDir while retaining original parent cwd. JSON kore-print invokes KAST_UTIL (Java) and outputs through /dev/stdout. Shell temp paths are writable under private /tmp; compiled data and initial cwd are only read by these inspected shell paths.

**limitation**. No K jar bytecode audit or full native interpreter/JVM write-set proof performed. JVM temp/performance/error logs, K Java caches or native error paths may still attempt writes; read-only root/HOME and256MiB temp capacity could cause failures. Source inspection alone cannot claim runtime completion or deny such writes.

Primary sources: [OpenJDK21+35 native properties](https://raw.githubusercontent.com/openjdk/jdk/jdk-21%2B35/src/java.base/unix/native/libjava/java_props_md.c), [SystemProps](https://raw.githubusercontent.com/openjdk/jdk/jdk-21%2B35/src/java.base/share/classes/jdk/internal/util/SystemProps.java). Exact fetched bodies, timestamps and SHA256 values are retained inside the companion JSON, along with local hashes.

Outstanding conditions:

- Before live admission bind/revalidate passwd and nsswitch bytes/ownership/ancestors under OS trust; ensure real as well as effective UID1000 and preserved readability in the exact final namespace. Protect fixed JVM argv and clean env against user.home overrides.
- Existing Opus user.home/direct-loader canary requirement remains outstanding; this source argument does not mechanically discharge an explicitly empirical condition. Any Java/K canary requires separately adopted bounded authority. No canary or extra retry is authorized here.
- Validate final harmless setup/FD/network/cwd/runner/timeout controls under existing authority. Actual Java/native ptrace and readonly filesystem compatibility remain unobserved here; reserve changes to diagnostic argv/output/resource limits for explicit reviewed amendments.
- The retained sole experiment targets a known crash boundary, not successful completion of the accepted-depth program. If separately authorized, loader reachability canary and term equivalence have distinct predicates. Neither successful Java startup nor accepted-depth completion replaces raw interpreter signal/exit classification. Preserve wrapper113, parser/output failure and inconclusive dispositions.

Only status, read-only filesystem/static ELF inspection and bounded public Scrapling fetches. No Java/ng/K/strace/compiler/native canary or product dispatch. Only these two outputs written. No cookies retained; existing raw-source/API fetching pattern sufficient, no skill changes.
