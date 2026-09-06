# ZKIR K semantics: build and check.
#   make check         kompile the five LLVM definitions, run the six check layers, write dated receipts
#   make kompile       the five LLVM definitions only
#   make zkir-symbolic kompile the Haskell definition (zkir-symbolic.k) the claims run against
#   make fast          the four quick layers (no differential runs)
#   make zkir-contract kompile the target-contract definition (zkir-contract-main.k) only
#   make contract      run the contract over the corpus and the Moriarty contexts, write a dated receipt
#   make contexts      the seven Moriarty transaction contexts on K, preprocess and the circuit oracle (M4)
#   make circuit       the two differential runs in --circuit mode (MockProver oracle), dated receipts
#   make provability   keygen over every parseable corpus program against the contract receipt of $(STAMP) (M5b)
#   make claims        the kprove claims on the Haskell definition; receipt and the kprove logs beside it
#   make drift         the pinned crates against the upstream heads
# Run from experiments/zkir-k; Python goes through uv from the repository root.
# Receipts are named <layer>-$(STAMP).txt with STAMP = $(DATE)$(SUFFIX); pass
# SUFFIX=b (c, d, ...) to keep an earlier receipt of the same day instead of
# overwriting it. Every recipe runs under bash with -e -u -o pipefail, so a
# failing tool fails its target even though its output goes through tee.
SHELL       := /bin/bash
.SHELLFLAGS := -eu -o pipefail -c
ROOT    := $(abspath ../..)
UV      := cd $(ROOT) && uv run --group zkir-k python
TOOLS   := experiments/zkir-k/tools
DATE    ?= $(shell date +%F)
SUFFIX  ?=
STAMP   := $(DATE)$(SUFFIX)
RECEIPT := $(ROOT)/evidence
DEFS    := zkir zkir-ext zkir-check zkir-test

.PHONY: check kompile fast differential contract contexts circuit provability claims drift zkir-contract zkir-symbolic $(DEFS)

kompile: $(DEFS) zkir-contract
$(DEFS):
	cd semantics && kompile $@.k --backend llvm --emit-json -O1
zkir-contract:
	cd semantics && kompile zkir-contract-main.k --backend llvm --emit-json -O1
zkir-symbolic:
	cd semantics && kompile zkir-symbolic.k --backend haskell

contract: zkir-contract
	$(UV) $(TOOLS)/contract_corpus.py  | tee $(RECEIPT)/zkir-k-contract-corpus-$(STAMP).txt

contexts: zkir zkir-ext
	$(UV) $(TOOLS)/moriarty_contexts.py --ext | tee $(RECEIPT)/zkir-k-moriarty-contexts-$(STAMP).txt

circuit: zkir zkir-ext
	$(UV) $(TOOLS)/diff_test.py --circuit       | tee $(RECEIPT)/zkir-k-circuit-differential-92e8bdd3-$(STAMP).txt
	$(UV) $(TOOLS)/diff_test.py --ext --circuit | tee $(RECEIPT)/zkir-k-circuit-differential-ext-2ffe2d1-$(STAMP).txt

provability:
	$(UV) $(TOOLS)/provability.py --contract-receipt $(RECEIPT)/zkir-k-contract-corpus-$(STAMP).txt --out $(RECEIPT)/zkir-k-provability-$(STAMP).txt

claims:
	$(UV) $(TOOLS)/run_claims.py --stamp $(STAMP) --log-dir $(RECEIPT)/zkir-k-claims-$(STAMP)-logs | tee $(RECEIPT)/zkir-k-claims-$(STAMP).txt

drift:
	$(UV) $(TOOLS)/upstream_drift.py | tee $(RECEIPT)/zkir-k-upstream-drift-$(STAMP).txt

fast: kompile
	$(UV) $(TOOLS)/unit_values.py      | tee $(RECEIPT)/zkir-k-unit-values-$(STAMP).txt
	$(UV) $(TOOLS)/unit_hash.py        | tee $(RECEIPT)/zkir-k-unit-hash-$(STAMP).txt
	$(UV) $(TOOLS)/check_corpus.py     | tee $(RECEIPT)/zkir-k-milestone2-corpus-check-$(STAMP).txt
	$(UV) $(TOOLS)/divergence_tests.py | tee $(RECEIPT)/zkir-k-divergence-tests-$(STAMP).txt

differential: kompile
	$(UV) $(TOOLS)/diff_test.py        | tee $(RECEIPT)/zkir-k-differential-92e8bdd3-$(STAMP).txt
	$(UV) $(TOOLS)/diff_test.py --ext  | tee $(RECEIPT)/zkir-k-differential-ext-2ffe2d1-$(STAMP).txt

# reached only when every layer above returned 0 (bash -e, pipefail)
check: fast differential
	@echo "all six check layers ran and returned 0; receipts under $(RECEIPT) with suffix $(STAMP)"
