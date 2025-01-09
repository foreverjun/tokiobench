.PHONY: clean
clean:
	cargo clean

	cd tokio; cargo clean
	cd tokio-metrics; cargo clean

.PHONY: plot
plot:
	cd graphs && uv sync && uv run main.py -ms -mt

.PHONY: check
check:
	cargo bench --no-run -F check
