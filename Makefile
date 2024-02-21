export CARGO_PROFILE_RELEASE_CODEGEN_UNITS = 1
export CARGO_PROFILE_RELEASE_LTO = fat
export DOCKER_BUILDKIT = 1
export CARGO_BUILD_RUSTFLAGS = -D warnings
export hkt_RELEASE_BUILD = no
export CARGO_TARGET_DIR = target


# By default, build a regular release
all: release


docker-hktcore: DOCKER_TAG ?= hktcore
docker-hktcore:
	docker build -t $(DOCKER_TAG) -f Dockerfile --build-arg=make_target=hktd-release         --progress=plain .

docker-hktcore-sandbox: DOCKER_TAG ?= hktcore-sandbox
docker-hktcore-sandbox:
	docker build -t $(DOCKER_TAG) -f Dockerfile --build-arg=make_target=hktd-sandbox-release --progress=plain .

docker-hktcore-nightly: DOCKER_TAG ?= hktcore-nightly
docker-hktcore-nightly:
	docker build -t $(DOCKER_TAG) -f Dockerfile --build-arg=make_target=hktd-nightly-release --progress=plain .


release: hktd-release
	cargo build -p store-validator --release
	cargo build -p genesis-populate --release
	$(MAKE) sandbox-release

hktd: hktd-release
	@echo 'hktd binary ready in ./target/release/hktd'

hktd-release: hkt_RELEASE_BUILD=release
hktd-release:
	cargo build -p hktd --release

hktd-debug:
	cargo build -p hktd

debug: hktd-debug
	cargo build -p store-validator
	cargo build -p genesis-populate
	$(MAKE) sandbox


perf-release: hkt_RELEASE_BUILD=release
perf-release:
	CARGO_PROFILE_RELEASE_DEBUG=true cargo build -p hktd --release --features performance_stats,memory_stats
	cargo build -p store-validator --release --features hktcore/performance_stats,hktcore/memory_stats


perf-debug:
	cargo build -p hktd --features performance_stats,memory_stats
	cargo build -p store-validator --features hktcore/performance_stats,hktcore/memory_stats


nightly-release: hktd-nightly-release
	cargo build -p store-validator --release --features hktcore/nightly,hktcore/performance_stats,hktcore/memory_stats
	cargo build -p genesis-populate --release --features hktcore/nightly,hktcore/performance_stats,hktcore/memory_stats

hktd-nightly-release:
	cargo build -p hktd --release --features nightly,performance_stats,memory_stats


nightly-debug:
	cargo build -p hktd --features nightly,performance_stats,memory_stats
	cargo build -p store-validator --features hktcore/nightly,hktcore/performance_stats,hktcore/memory_stats
	cargo build -p genesis-populate --features hktcore/nightly,hktcore/performance_stats,hktcore/memory_stats


assertions-release: hkt_RELEASE_BUILD=release
assertions-release:
	CARGO_PROFILE_RELEASE_DEBUG=true CARGO_PROFILE_RELEASE_DEBUG_ASSERTIONS=true cargo build -p hktd --release --features performance_stats,memory_stats


sandbox: CARGO_TARGET_DIR=sandbox
sandbox: hktd-sandbox
	mkdir -p target/debug
	ln -f sandbox/debug/hktd target/debug/hktd-sandbox
	@ln -f sandbox/debug/hktd target/debug/hkt-sandbox

hktd-sandbox:
	cargo build -p hktd --features sandbox


sandbox-release: CARGO_TARGET_DIR=sandbox
sandbox-release: hktd-sandbox-release
	mkdir -p target/release
	ln -f sandbox/release/hktd target/release/hktd-sandbox
	@ln -f sandbox/release/hktd target/release/hkt-sandbox

hktd-sandbox-release:
	cargo build -p hktd --features sandbox --release

shardnet-release:
	cargo build -p hktd --release --features shardnet


.PHONY: docker-hktcore docker-hktcore-nightly release hktd debug
.PHONY: perf-release perf-debug nightly-release nightly-debug assertions-release sandbox
.PHONY: sandbox-release
