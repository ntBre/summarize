TESTFLAGS = --nocapture --test-threads=1
ARGS =

TARGET = target/x86_64-unknown-linux-gnu/release/summarize-bin

test:
	RUST_BACKTRACE=1 cargo test -- ${TESTFLAGS} ${ARGS}

build:
	RUSTFLAGS='-C target-feature=+crt-static' \
        cargo build -p summarize-bin --release --target x86_64-unknown-linux-gnu

woods: build
	scp -C $(TARGET) 'woods:bin/rsummarize'

install:
	ln -s $(realpath $(TARGET)) /usr/bin/rsummarize
