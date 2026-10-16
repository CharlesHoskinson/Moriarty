# probe-k6.sh - bounded Q6N/Q6P capability probe, one text for both views.
# Invocation (caller duty, inside the view):
#   /usr/bin/bash --noprofile --norc -c TEXT probe-k6 T5 u p n i s
# TEXT is the exact byte content of this file. $0 is probe-k6, $# is 6:
# $1 is the literal retained host T5 path, $2..$6 are the host user, pid,
# net, ipc and uts namespace strings captured by the launcher immediately
# before launch. On $1 the only operation is test ! -e: an in-view
# metadata existence test, evaluated where that host path is not mounted
# and must be absent; it opens and lists no host T5 content. No mori-k2
# command is executed.
# No key bytes are opened, read, copied or hashed; the proving key is
# reached only by directory listing and stat/lstat metadata. Nothing is
# written. The probe exits 0 only after printing probe-ok; any missing,
# malformed or non-zero gated predicate prints one PROBE-FAIL record and
# exits 1. Allowed reads: /proc/self/status via an exec'd child,
# /proc/self/ns/*, /proc/self/mountinfo, /proc/self/fd, /proc/net/dev,
# listings of /, /etc, /task, /task/bin, /task/in, /task/out, /task/stage
# and, in Q6P, /task/keys-ro, plus stat/lstat metadata for those paths,
# and the loader/runtime metadata read implicitly when the /usr/bin
# tools below execute. Device nodes are tested for character type only
# and are never read. Only /usr/bin tools and shell builtins are used.
set -u
fail() { printf 'PROBE-FAIL %s\n' "$*"; exit 1; }
test "$#" -eq 6 || fail "gate=usage argc=$# expected=6"
HOST_T5="$1"
HOST_USER="$2"
HOST_PID="$3"
HOST_NET="$4"
HOST_IPC="$5"
HOST_UTS="$6"
if test -e /task/keys-ro || test -L /task/keys-ro; then MODE=Q6P; else MODE=Q6N; fi
printf 'probe-start mode=%s\n' "$MODE"
printf 'record mode=%s\n' "$MODE"
# (a) argument count is gated above; in-view uid must be non-zero.
UIDV="$(/usr/bin/id -u)" || fail "gate=uid tool=id failed"
case "$UIDV" in ''|*[!0-9]*) fail "gate=uid malformed uid=$UIDV" ;; esac
test "$UIDV" != "0" || fail "gate=uid uid=0"
printf 'record uid=%s\n' "$UIDV"
# (b) capabilities from an exec'd child (/usr/bin/cat of its own status),
# which represents a native exec. CapInh, CapPrm, CapEff and CapAmb must
# be all zero. Every Cap line plus NoNewPrivs and Seccomp is recorded;
# a missing or malformed line fails closed.
CHILD_STATUS="$(/usr/bin/cat /proc/self/status)" || fail "gate=cap child-status unreadable"
CapInh=""; CapPrm=""; CapEff=""; CapBnd=""; CapAmb=""; NoNewPrivsV=""; SeccompV=""
while IFS= read -r line; do
  case "$line" in
    CapInh:*) CapInh="${line##*[[:space:]]}" ;;
    CapPrm:*) CapPrm="${line##*[[:space:]]}" ;;
    CapEff:*) CapEff="${line##*[[:space:]]}" ;;
    CapBnd:*) CapBnd="${line##*[[:space:]]}" ;;
    CapAmb:*) CapAmb="${line##*[[:space:]]}" ;;
    NoNewPrivs:*) NoNewPrivsV="${line##*[[:space:]]}" ;;
    Seccomp:*) SeccompV="${line##*[[:space:]]}" ;;
  esac
done <<< "$CHILD_STATUS"
for pair in "CapInh=$CapInh" "CapPrm=$CapPrm" "CapEff=$CapEff" "CapBnd=$CapBnd" "CapAmb=$CapAmb"; do
  fname="${pair%%=*}"; fval="${pair#*=}"
  test -n "$fval" || fail "gate=cap missing field=$fname"
  if [[ ! "$fval" =~ ^[0-9a-f]{16}$ ]]; then fail "gate=cap malformed field=$fname value=$fval"; fi
  printf 'record cap %s=%s\n' "$fname" "$fval"
done
test "$CapInh" = "0000000000000000" || fail "gate=cap nonzero field=CapInh value=$CapInh"
test "$CapPrm" = "0000000000000000" || fail "gate=cap nonzero field=CapPrm value=$CapPrm"
test "$CapEff" = "0000000000000000" || fail "gate=cap nonzero field=CapEff value=$CapEff"
test "$CapAmb" = "0000000000000000" || fail "gate=cap nonzero field=CapAmb value=$CapAmb"
case "$NoNewPrivsV" in ''|*[!0-9]*) fail "gate=status malformed field=NoNewPrivs value=$NoNewPrivsV" ;; esac
case "$SeccompV" in ''|*[!0-9]*) fail "gate=status malformed field=Seccomp value=$SeccompV" ;; esac
printf 'record status NoNewPrivs=%s\n' "$NoNewPrivsV"
printf 'record status Seccomp=%s\n' "$SeccompV"
# (c) each in-view namespace differs from the host value supplied in argv.
ns_names=(user pid net ipc uts)
ns_hosts=("$HOST_USER" "$HOST_PID" "$HOST_NET" "$HOST_IPC" "$HOST_UTS")
idx=0
while test "$idx" -lt 5; do
  n="${ns_names[$idx]}"; h="${ns_hosts[$idx]}"
  IN="$(/usr/bin/readlink "/proc/self/ns/$n")" || fail "gate=ns unreadable ns=$n"
  if [[ ! "$IN" =~ ^${n}:\[[0-9]+\]$ ]]; then fail "gate=ns malformed view ns=$n value=$IN"; fi
  if [[ ! "$h" =~ ^${n}:\[[0-9]+\]$ ]]; then fail "gate=ns malformed host ns=$n value=$h"; fi
  test "$IN" != "$h" || fail "gate=ns host-equal ns=$n value=$IN"
  printf 'record ns %s view=%s host=%s\n' "$n" "$IN" "$h"
  idx=$((idx + 1))
done
# (d) /proc/net/dev lists exactly one interface and it is lo.
mapfile -t ND < /proc/net/dev || fail "gate=netdev unreadable"
test "${#ND[@]}" -eq 3 || fail "gate=netdev lines=${#ND[@]} expected=3"
IFACE="${ND[2]%%:*}"; IFACE="${IFACE//[[:space:]]/}"
test "$IFACE" = "lo" || fail "gate=netdev interface=$IFACE expected=lo"
printf 'record netdev interfaces=1 name=lo\n'
# (e) host task root and host-only paths are absent in-view.
test ! -e "$HOST_T5" || fail "gate=absent path=host-t5"
for p in /home /root /run /var /etc/alternatives /nonexistent; do
  test ! -e "$p" || fail "gate=absent path=$p"
  printf 'record absent path=%s\n' "$p"
done
# (f) root entries are the exact set, with or without etc; /etc, when
# present, holds only ld.so.cache.
ROOTL="$(/usr/bin/ls -A / | LC_ALL=C /usr/bin/sort | /usr/bin/tr '\n' ' ')" || fail "gate=root listing failed"
printf 'record root entries=%s\n' "$ROOTL"
case "$ROOTL" in
  "bin dev etc lib lib64 proc task tmp usr "|"bin dev lib lib64 proc task tmp usr ") ;;
  *) fail "gate=root entries=$ROOTL" ;;
esac
if test -e /etc; then
  ETCL="$(/usr/bin/ls -A /etc | LC_ALL=C /usr/bin/sort | /usr/bin/tr '\n' ' ')" || fail "gate=etc listing failed"
  printf 'record etc entries=%s\n' "$ETCL"
  test "$ETCL" = "ld.so.cache " || fail "gate=etc entries=$ETCL"
else
  printf 'record etc entries=absent\n'
fi
# (g) exact /task shape for the inferred mode; binary and input sets.
test ! -L /task || fail "gate=task link path=/task"
TASKL="$(/usr/bin/ls -A /task | LC_ALL=C /usr/bin/sort | /usr/bin/tr '\n' ' ')" || fail "gate=task listing failed"
printf 'record task entries=%s\n' "$TASKL"
if test "$MODE" = "Q6P"; then
  test "$TASKL" = "bin in keys-ro out stage " || fail "gate=task entries=$TASKL mode=Q6P"
else
  test "$TASKL" = "bin in out stage " || fail "gate=task entries=$TASKL mode=Q6N"
fi
for d in /task/bin /task/in /task/out /task/stage; do
  test ! -L "$d" || fail "gate=task link path=$d"
  test -d "$d" || fail "gate=task notdir path=$d"
done
BINL="$(/usr/bin/ls -A /task/bin | LC_ALL=C /usr/bin/sort | /usr/bin/tr '\n' ' ')" || fail "gate=bin listing failed"
test "$BINL" = "mori-k2 " || fail "gate=bin entries=$BINL"
test -f /task/bin/mori-k2 || fail "gate=bin not-regular"
test ! -L /task/bin/mori-k2 || fail "gate=bin link"
test -x /task/bin/mori-k2 || fail "gate=bin not-executable"
test ! -u /task/bin/mori-k2 || fail "gate=bin setuid"
test ! -g /task/bin/mori-k2 || fail "gate=bin setgid"
printf 'record bin entries=mori-k2 type=regular executable=1 setid=0\n'
INL="$(/usr/bin/ls -A /task/in | LC_ALL=C /usr/bin/sort | /usr/bin/tr '\n' ' ')" || fail "gate=in listing failed"
printf 'record in entries=%s\n' "$INL"
test "$INL" = "K4H-GOLD.json conditional-reserve.mori installment-invoice.mori invoice-partial.request.json invoice-partial.selector.json invoice-partial.setup.json " || fail "gate=in entries=$INL"
for f in installment-invoice.mori conditional-reserve.mori invoice-partial.selector.json invoice-partial.request.json invoice-partial.setup.json K4H-GOLD.json; do
  test -f "/task/in/$f" || fail "gate=in not-regular name=$f"
  test ! -L "/task/in/$f" || fail "gate=in link name=$f"
done
OUTL="$(/usr/bin/ls -A /task/out)" || fail "gate=out listing failed"
test -z "$OUTL" || fail "gate=out nonempty"
STGL="$(/usr/bin/ls -A /task/stage)" || fail "gate=stage listing failed"
test -z "$STGL" || fail "gate=stage nonempty"
printf 'record out entries=empty\n'
printf 'record stage entries=empty\n'
# (h) last mount at each point in /proc/self/mountinfo decides ro or rw;
# shell test -w (access) must agree with the mount options.
mnt_hit() {
  local want="$1" hit="" mid par mdev mroot mp mrest
  while read -r mid par mdev mroot mp mrest; do
    if test "$mp" = "$want"; then hit="$mrest"; fi
  done < /proc/self/mountinfo
  printf '%s' "$hit"
}
check_mount() {
  local point="$1" want="$2" hit opts
  hit="$(mnt_hit "$point")"
  test -n "$hit" || fail "gate=mount missing point=$point"
  opts="${hit%% *}"
  printf 'record mount point=%s options=%s\n' "$point" "$opts"
  case "$want" in
    ro) case ",$opts," in *,ro,*) ;; *) fail "gate=mount not-ro point=$point options=$opts" ;; esac
        test ! -w "$point" || fail "gate=mount writable point=$point" ;;
    rw) case ",$opts," in *,rw,*) ;; *) fail "gate=mount not-rw point=$point options=$opts" ;; esac
        test -w "$point" || fail "gate=mount not-writable point=$point" ;;
  esac
}
for d in /usr /bin /lib /lib64 /task/bin /task/in /task/stage; do check_mount "$d" ro; done
if test "$MODE" = "Q6P"; then check_mount /task/keys-ro ro; fi
check_mount /task/out rw
if test "$MODE" = "Q6P"; then
  test ! -w /task/keys-ro/invoice-partial.pk || fail "gate=mount writable point=/task/keys-ro/invoice-partial.pk"
fi
# (i) /tmp is tmpfs; the three device nodes are character devices only.
TMPhit="$(mnt_hit /tmp)"; test -n "$TMPhit" || fail "gate=mount missing point=/tmp"
case "$TMPhit" in *" - "*) ;; *) fail "gate=mount malformed point=/tmp" ;; esac
TMPtail="${TMPhit#* - }"; TMPfs="${TMPtail%% *}"
printf 'record mount point=/tmp fstype=%s\n' "$TMPfs"
test "$TMPfs" = "tmpfs" || fail "gate=mount fstype point=/tmp value=$TMPfs"
for dev in /dev/null /dev/urandom /dev/random; do
  test -c "$dev" || fail "gate=dev not-char path=$dev"
  printf 'record dev path=%s type=char read=never\n' "$dev"
done
# (j) stdin is /dev/null in the parent; an exec'd child listing its own
# /proc/self/fd sees exactly 0 1 2 3, where 3 is its directory handle.
FD0="$(/usr/bin/readlink /proc/self/fd/0)" || fail "gate=fd stdin unreadable"
test "$FD0" = "/dev/null" || fail "gate=fd stdin=$FD0 expected=/dev/null"
printf 'record fd stdin=%s\n' "$FD0"
CHILD_FD_RAW="$(/usr/bin/ls /proc/self/fd)" || fail "gate=fd child listing failed"
CHILD_FD="$(printf '%s' "$CHILD_FD_RAW" | /usr/bin/tr '\n' ' ')"
test "$CHILD_FD" = "0 1 2 3" || fail "gate=fd childfds=$CHILD_FD expected=0-1-2-3"
printf 'record fdlist child=0,1,2,3\n'
# (k) environment names are a subset of the cleared set and the three
# pinned values are exact. Names are recorded in sorted order.
ENAMES="$(compgen -e | LC_ALL=C /usr/bin/sort)" || fail "gate=env enumeration failed"
test -n "$ENAMES" || fail "gate=env empty"
while IFS= read -r vname; do
  test -n "$vname" || continue
  case "$vname" in HOME|PATH|PWD|SHLVL|OLDPWD|_) ;; *) fail "gate=env unexpected name=$vname" ;; esac
  printf 'record env %s=%s\n' "$vname" "${!vname}"
done <<< "$ENAMES"
test "${HOME:-}" = "/nonexistent" || fail "gate=env HOME=${HOME:-unset}"
test "${PATH:-}" = "/usr/bin:/bin" || fail "gate=env PATH=${PATH:-unset}"
test "${PWD:-}" = "/task" || fail "gate=env PWD=${PWD:-unset}"
# (l) mode-specific key gates. No key byte is opened in either mode.
if test "$MODE" = "Q6N"; then
  test ! -e /task/keys-ro || fail "gate=keys present-in-Q6N"
  printf 'record keys state=absent\n'
else
  test ! -L /task/keys-ro || fail "gate=keys link path=/task/keys-ro"
  test -d /task/keys-ro || fail "gate=keys notdir path=/task/keys-ro"
  KDMODE="$(/usr/bin/stat -c %a /task/keys-ro)" || fail "gate=keys stat-dir failed"
  test "$KDMODE" = "700" || fail "gate=keys dirmode=$KDMODE expected=700"
  KEYL="$(/usr/bin/ls -A /task/keys-ro | LC_ALL=C /usr/bin/sort | /usr/bin/tr '\n' ' ')" || fail "gate=keys listing failed"
  printf 'record keys entries=%s\n' "$KEYL"
  test "$KEYL" = "invoice-partial.pk " || fail "gate=keys entries=$KEYL"
  PK=/task/keys-ro/invoice-partial.pk
  test ! -L "$PK" || fail "gate=pk link"
  test -f "$PK" || fail "gate=pk not-regular"
  KTUPLE="$(/usr/bin/stat -c '%d:%i:%s:%a:%h:%Y:%Z' "$PK")" || fail "gate=pk stat failed"
  IFS=':' read -r kdev kino ksize kmode knlink kmtime kctime <<< "$KTUPLE"
  for kv in "$kdev" "$kino" "$ksize" "$kmode" "$knlink" "$kmtime" "$kctime"; do
    case "$kv" in ''|*[!0-9]*) fail "gate=pk malformed tuple=$KTUPLE" ;; esac
  done
  test "$kmode" = "600" || fail "gate=pk mode=$kmode expected=600"
  test "$ksize" = "98715456" || fail "gate=pk size=$ksize expected=98715456"
  test "$knlink" = "1" || fail "gate=pk nlink=$knlink expected=1"
  printf 'record keytuple=%s\n' "$KTUPLE"
fi
printf 'probe-ok mode=%s\n' "$MODE"
exit 0
