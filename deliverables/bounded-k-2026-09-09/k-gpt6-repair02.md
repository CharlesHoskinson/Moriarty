# GPT-6 minimal CLI repair and second-allocation review

Verdict: **approve the exact minimal repair and one new bounded allocation under resource-proposal-02.json.** Separate exact Fable approval and root dispatch checks remain required. This does not refund or erase the first attempt.

Candidate: `candidate-02.json`, SHA-256 `d6f3caffa797d1c8eac0de5a6a08809c591a2183f005d4d71a9254e880359aa8`. I refreshed the required development workflow/status and verified all 28 candidate/supporting bindings. No K command ran during this review.

The retained actual compile command exited 1 after 0.415895866 seconds. Its diagnostic identifies `-O0` as an unsupported second positional parameter. The supervisor exited 2 after 0.485947609 seconds. No definition compiled and no krun invocation occurred. Earlier help output did not validate ordinary argument parsing. The failure receipts remain under `attempt-01/`.

The runner change removes only `-O0`. Reinserting that element reproduces the prior runner SHA-256. Independent AST extraction of the new argv exactly matches the failed argv with that single element removed. The pinned K frontend defines only the -O1/-O2/-O3 optimization booleans. Its LLVM backend forwards those flags only when selected. Omitting them preserves the intended default optimization route. This source inspection does not claim a successful compile.

The other product changes are the README default-optimization wording and an ignore entry for preserved `.build-attempt-*/` directories. Removing that one ignore line reproduces the prior ignore-file hash. K rules, codec, fixtures, tests, toolchain and financial reference inputs remain unchanged. The new resource receipt is the only added candidate input.

The new allocation is justified by the observed argument-parsing failure and a specific compatible command change. Approve one compile at most 180 seconds, up to 16 krun invocations at most 20 seconds each, 512 seconds aggregate, 4 GiB for the entire process tree, zero swap and one heavy tree. The unit02 command retains nonblocking flock, whole-cgroup accounting and immediate SIGKILL. The first allocation remains consumed: one compile, zero krun. This is a separately reviewed next allocation, not an automatic retry.

After both reviews, root may preserve the old build directory by renaming it intact to `.build-attempt-01`. Do not delete its attempt marker or historical charges. Root must verify candidate bytes, no overlapping heavy workloads, and the exact reviewed systemd command before dispatch. Stop on the first failure and retain raw outputs and resource observations. Any further repair or allocation requires review.

All earlier scope limits remain. No K compatibility, trace agreement, proof, full source/Core correspondence, SP03 completion, native PCD or ledger acceptance is established. Later independent result audits remain required.
