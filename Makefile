TESTFLAGS = --nocapture --test-threads=1
ARGS =

TARGET = target/x86_64-unknown-linux-gnu/release/summarize-bin

clippy:
	cargo clippy --tests --all

test:
	RUST_BACKTRACE=1 cargo test --workspace --no-fail-fast -- ${TESTFLAGS} ${ARGS}

build:
	RUSTFLAGS='-C target-feature=+crt-static' \
        cargo build -p summarize-bin --release --target x86_64-unknown-linux-gnu

try:
	cargo run -p summarize-bin testfiles/spectro.out testfiles/c2h4.out testfiles/allyl.out ${ARGS}

run:
	cargo run -p summarize-bin ${ARGS}

woods: build
	scp -C $(TARGET) 'woods:bin/rsummarize'

install:
	ln -s $(realpath $(TARGET)) /usr/bin/rsummarize
