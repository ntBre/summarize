TESTFLAGS = --nocapture --test-threads=1
ARGS =

test:
	RUST_BACKTRACE=1 cargo test -- ${TESTFLAGS} ${ARGS}

build:
	RUSTFLAGS='-C target-feature=+crt-static' \
        cargo build -p summarize-bin --release --target x86_64-unknown-linux-gnu
