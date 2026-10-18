# Existing sandbox dependency probes

Both probes executed `/bin/true` using installed `timeout`, `sudo`, `prlimit`,
`setpriv`, `env` and `bwrap`, with UID/GID1000, no capabilities and the selected
namespace isolation. The baseline exited0 in0.016203 seconds; the masked variant
exited0 in0.016069 seconds. Both captured zero output bytes. Their exact argv,
resource bounds and raw results are preserved here.

The masked variant first inspected metadata for the six exact mask paths. All
six were directories. It added empty read-only directory mounts without reading
their contents. Both probes used ephemeral namespaces and no SDK, Cargo, circuit,
proof, wallet or retired native execution root.

These checks establish basic namespace and mask startup on this host. They do
not exercise the sealed-memfd trampoline, seccomp listener handoff, exact native
runner mount layout or compiler. The original A0 failure remains unresolved.
The designated owner supplied a separate diagnostic source proposal; it needs
independent source review before execution. No unchanged native attempt was
repeated, and the former tmpfs mounts/cgroups remain torn down.
