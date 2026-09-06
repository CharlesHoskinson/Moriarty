# ZKIR K semantics: build and check.
#   make check        kompile the four definitions, run the six check layers, write dated receipts
#   make kompile      the four definitions only
#   make fast         the four quick layers (no differential runs)
# Run from experiments/zkir-k; Python goes through uv from the repository root.
ROOT    := $(abspath ../..)
UV      := cd $(ROOT) && uv run --group zkir-k python
TOOLS   := experiments/zkir-k/tools
DATE    ?= $(shell date +%F)
RECEIPT := $(ROOT)/evidence
DEFS    := zkir zkir-ext zkir-check zkir-test

.PHONY: check kompile fast differential $(DEFS)

kompile: $(DEFS)
$(DEFS):
	cd semantics && kompile $@.k --backend llvm --emit-json -O1

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
