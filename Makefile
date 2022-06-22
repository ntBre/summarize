TESTFLAGS = --nocapture --test-threads=1
ARGS =

test:
	RUST_BACKTRACE=1 cargo test -- ${TESTFLAGS} ${ARGS}
