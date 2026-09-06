# ZKIR K semantics: build and check.
#   make check         kompile the five definitions, run the six check layers, write dated receipts
#   make kompile       the five definitions only
#   make fast          the four quick layers (no differential runs)
#   make zkir-contract kompile the target-contract definition (zkir-contract-main.k) only
#   make contract      run the contract over the corpus and the Moriarty contexts, write a dated receipt
#   make contexts      the seven Moriarty transaction contexts on K, preprocess and the circuit oracle (M4)
#   make circuit       the two differential runs in --circuit mode (MockProver oracle), dated receipts
#   make provability   unknown-witness keygen over the corpus against the contract receipt of $(DATE) (M5b)
# Run from experiments/zkir-k; Python goes through uv from the repository root.
ROOT    := $(abspath ../..)
UV      := cd $(ROOT) && uv run --group zkir-k python
TOOLS   := experiments/zkir-k/tools
DATE    ?= $(shell date +%F)
RECEIPT := $(ROOT)/evidence
DEFS    := zkir zkir-ext zkir-check zkir-test

.PHONY: check kompile fast differential contract contexts circuit provability zkir-contract $(DEFS)

kompile: $(DEFS) zkir-contract
$(DEFS):
	cd semantics && kompile $@.k --backend llvm --emit-json -O1
zkir-contract:
	cd semantics && kompile zkir-contract-main.k --backend llvm --emit-json -O1

contract: zkir-contract
	$(UV) $(TOOLS)/contract_corpus.py  | tee $(RECEIPT)/zkir-k-contract-corpus-$(DATE).txt

contexts: zkir zkir-ext
	$(UV) $(TOOLS)/moriarty_contexts.py --ext | tee $(RECEIPT)/zkir-k-moriarty-contexts-$(DATE).txt

circuit: zkir zkir-ext
	$(UV) $(TOOLS)/diff_test.py --circuit       | tee $(RECEIPT)/zkir-k-circuit-differential-92e8bdd3-$(DATE).txt
	$(UV) $(TOOLS)/diff_test.py --ext --circuit | tee $(RECEIPT)/zkir-k-circuit-differential-ext-2ffe2d1-$(DATE).txt

provability:
	$(UV) $(TOOLS)/provability.py --contract-receipt $(RECEIPT)/zkir-k-contract-corpus-$(DATE).txt --out $(RECEIPT)/zkir-k-provability-$(DATE).txt

fast: kompile
	$(UV) $(TOOLS)/unit_values.py      | tee $(RECEIPT)/zkir-k-unit-values-$(DATE).txt
	$(UV) $(TOOLS)/unit_hash.py        | tee $(RECEIPT)/zkir-k-unit-hash-$(DATE).txt
	$(UV) $(TOOLS)/check_corpus.py     | tee $(RECEIPT)/zkir-k-milestone2-corpus-check-$(DATE).txt
	$(UV) $(TOOLS)/divergence_tests.py | tee $(RECEIPT)/zkir-k-divergence-tests-$(DATE).txt

differential: kompile
	$(UV) $(TOOLS)/diff_test.py        | tee $(RECEIPT)/zkir-k-differential-92e8bdd3-$(DATE).txt
	$(UV) $(TOOLS)/diff_test.py --ext  | tee $(RECEIPT)/zkir-k-differential-ext-2ffe2d1-$(DATE).txt

check: fast differential
	@echo "all six check layers ran; receipts under $(RECEIPT) with suffix $(DATE)"
