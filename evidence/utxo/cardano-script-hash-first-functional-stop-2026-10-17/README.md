# First functional script-hash run: stopped

The first admitted run executed 19 methods. Eighteen passed; `test_hex_and_type_inputs` failed its `mixed-case` subtest because `ScriptHashInputError` was not raised. The process exited 1 with zero errors, empty stdout and unchanged input pins, modes and listings. The CIP vector and all-sixteen-handler hash checks passed.

Both independent Opus reviews had predicted 19 passes. Their complete frozen judgments remain here beside the actual failure. The run was stopped with no retry, adoption or builder reliance. Opus source diagnosis is separate work; this snapshot makes no claim about the cause or a fix.

The result recorder counts complete `ok` method lines as 18; the unittest transcript records 19 methods and one failed subtest. No native or ledger operation occurred.
