import sys, pathlib
root = pathlib.Path(sys.argv[1])
p = root / "crates/sherlock-lfr/src/iface.rs"
s = p.read_text()
old = "&[ops[0], crate::fr::pack(run_tag), Fr::from_u64(k), ops[1], ops[2], link, Fr::from_u64(j)],"
assert s.count(old) == 1
s = s.replace(old, "&[ops[0], crate::fr::pack(run_tag), Fr::from_u64(k), ops[1], ops[2], { let _ = link; link_of(&crate::fp3::vd_nil()) }, Fr::from_u64(j)],")
p.write_text(s)
p = root / "crates/sherlock-lfr/src/lverify.rs"
s = p.read_text()
old = "    for (x, jj, tag, link) in [(a_in, jm1, f.tag_in, l_in), (a_out, jf, f.tag_out, l_out)] {"
assert s.count(old) == 1
s = s.replace(old, "    // FBBR2 MUTANT: the lock ignores the FP-3 link (the 6-operand form keeps a constant)\n    let nil_m = g.tape.fr_const(crate::iface::link_of(&fp3::vd_nil()));\n    let (l_in, l_out) = { let _ = (l_in, l_out); (nil_m, nil_m) };\n" + old)
p.write_text(s)
print("mutated")
