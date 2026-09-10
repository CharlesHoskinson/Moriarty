# LOCAL loan historical state readback proposal

Source preparation only. The completed financial run's182 frozen pins remain
unchanged. Both actual-result reviews require independent historical native state
readback before advancing to swap. No service was started for this preparation.

The probe uses the existing createLocalRpc transport (strict loopback, no redirects,
64KiB response cap and post-await deadlines) and actual pinned protocol ContractState.
It queries the explicit recorded hashes at20455/20459/20463/20467, checks them against
canonical blockHash and a fresh finalized header, then retains full state bytes,
all compiled fields, complete native balances and exact verifier-byte hashes.
The deployment's native initial state is independently compared byte-for-byte.
Absent state/pruned history/API failure yields UNKNOWN without fallback or retry.

Existing primary node RPC source explicitly passes the supplied at hash to the
runtime get_contract_state API; omitted at uses best block, which this probe never
uses. node-rpc-source.rs/json preserve the inspected public source. This does not
prove the pinned Docker image derives from that checkout. The retained published
local-finalized-state-02 and after-rejection-readback-01 observations establish
actual method use on the existing local node, including historical-anchor reads;
they do not establish availability of these four new anchors in advance.

The borrowed executor keeps independent timer semantics, exact source/admission/
review pin checks, installed SDK pins, unchanged images/limits, exclusive attempt
and argv/result retention. New timer240s plus31s cleanup fits300s total. Exactly
node and indexer start; proof server remains stopped. No wallet or private store
is loaded. Resource proposal uses the same20GiB available-memory gate; a failed
gate causes no service start. No permission to repeat the financial loan is created.

Five controlled tests pass with the real installed native decoder and retained
public compiled build; transport is injected and no HTTP service is contacted.
They exercise17-query positive capture, wrong canonical anchor, absent historical
state/no fallback, complete field/verifier mismatches and late final response.
These fixture states come from an older public loan and do not stand in for the
new actual historical states. Five inert AST activation cases plus undefined-name,
exact attempt-record and timer checks pass. Python compilation is source only.

Query result records retain normalized JSON values and hashes, not original HTTP
response bytes. Full native serialized state is retained exactly. Native explicit
zero balances remain in observations; comparison treats omitted summary zero and
native zero as equal, as the existing financial comparator does. No authenticated
state proof, independent native proof verification or finality beyond trusted RPC
is claimed. Outer containment, actual observed result and both result reviews are
separate obligations after source/resource admission.
