# one script for run benches with zero deps
# json file for constant sharing

.PHONY: bench
bench:
	python3 bench.py

.PHONY: clean
clean:
	cargo clean

	cd tokio; cargo clean
	cd tokio-metrics; cargo clean

.PHONY: cfgup
cfgup:
	echo TODO # update config via
