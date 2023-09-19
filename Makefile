.DEFAULT_GOAL:=help

SRC:=$(wildcard src/*.rs) Dockerfile Cargo.toml Makefile
export BIN:=trace-debug

# export BASE:=rust:1-slim-buster
export BASE:=rust:1-alpine3.18

trace-debug: $(SRC) ## Build binary with image '${BASE}'
	docker build --build-arg BASE=$(BASE) -t $(BIN) .
	docker create --name $(BIN) $(BIN)
	docker cp $(BIN):/app/target/release/trace-debug $(BIN)
	sleep 1
	docker rm $(BIN)

export NS?=default
export POD?=somepod
export CMD?=-e stdout
# export CMD?=-n 5 -e jaeger --host $$JAEGER_AGENT_HOST

.PHONY: exec
exec: $(BIN) ## Run in pod '${NS}/${POD}' command '${BIN} ${CMD}'
	kubectl -n $(NS) cp $(BIN) $(POD):/$(BIN)
	kubectl -n $(NS) exec $(POD) -- sh -c '/$(BIN) $(CMD)'

.PHONY: run
run: ## Compile and run locally command '${BIN} ${CMD}'
	cargo run -- $(CMD)

help: ## Show this help
	$(eval HELP_COL_WIDTH:=13)
	@echo "Makefile targets:"
	@grep -E '[^\s]+:.*?## .*$$' ${MAKEFILE_LIST} | grep -v grep | envsubst | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-${HELP_COL_WIDTH}s\033[0m %s\n", $$1, $$2}'
