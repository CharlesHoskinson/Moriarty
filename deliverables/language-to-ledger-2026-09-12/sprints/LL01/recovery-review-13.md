Independent bounded LL01 source triage; not a full-candidate audit.

S12b-1: Not applicable to exact fixed argv; preserve if argv changes.

recordedArgv explicitly contains --no-expand-macros, --output json, --parser expression-parser.py; no search/pattern/statistics/debug/version flags. krun:283-284,287-300,309-312 select these flags; configVars.sh declares only PGM; backend.txt is llvm. krun:527-542 uses explicit parser, :567 invokes llvm-krun with korefile and --dry-run -nm, :570-575 selects cp, :594-630 avoids search/match, :714 selects kore-print json. kore-print:218-231 sends json to absolute bin-unwrapped/kast, never bare kprint. llvm-krun:25-27 defaults text/no pretty; :157-158 disables macros; :223-224 selects dry-run; :443-480 constructs text, cp and cat to output then exits before interpreter/mv/kprint. Reached helper commands dirname/mktemp/rm/cat/cp already have krun command selections. No IO/STDIN declared, so awk route excluded.

S12b-2: Confirmed missing selected interpreter pin; source repair required.

bin-unwrapped/kast:1 selects /nix/store/306znyj77fv49kwnkpxmb0j2znqpa8bj-bash-5.2p26/bin/sh; :2 invokes lib/kframework/k. Parser execs pinned bin/kast at expression-parser.py:10, so sh reached before backend, independent of successful output. Diagnostic verify_wrapper_and_parser:239-335 checks parser shebang but does not require sh. verify_selected:200-213 only validates listed entries. Actual sh symlinkTarget=bash, resolvedTarget=/nix/store/306znyj77fv49kwnkpxmb0j2znqpa8bj-bash-5.2p26/bin/bash, resolved sha256=b5aea2266b8f20e705a5bba71998c6eb40adaaf40a2c0986b83a01b3ac4c7941.

S12b-4: Snapshot fixture identity concern bounded and rerun against canonical parsed snapshot; annotation remains.

Canonical diagnostic and candidate-source-12.py both hash 675d8e04d1ea4c93dd3dad05e2cbb299b9c4f88ed29c3aca7d91783ca7223876. Full parsed snapshot-inventory-draft-03.json differs from canonical snapshot only in scope string; all 506 file entries, ordering, hashes, modes, sizes, identity and limit fields equal. Draft raw sha256 fd72c4e8778608f54221af17d0fc6da3b1bc93f01290c61f2e6829981122e208; canonical raw sha256 d8c84c571399d445debf6948292550d56c67b1610e4d5dc830cc02dba70a7128. Independently imported existing regression module, replaced its snapshot variable with full canonical parsed object, and ran its 14 tests without executing __main__ or overwriting historical results: pass 14, zero errors/failures. Tests reserialize temporary manifests with corresponding digests; this does not claim historical test read canonical raw bytes. _inventory_union:676 return annotation should become tuple[dict[str,str],dict] consistent with actual tuple result.

Exact reproducible commands, repair expectations and negative cases are in recovery-review-13.json. Shell regression remains red; canonical-object snapshot controls pass 14/14. No native K, Java, strace, compile, bwrap, financial or network action ran.
