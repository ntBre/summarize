[![check](https://github.com/ntBre/summarize/actions/workflows/check.yml/badge.svg)](https://github.com/ntBre/summarize/actions/workflows/check.yml)
[![test](https://github.com/ntBre/summarize/actions/workflows/test.yml/badge.svg)](https://github.com/ntBre/summarize/actions/workflows/test.yml)

# summarize
summarize the output of chemistry programs

# Installation

Installation is as simple as running the following commands in your shell (after
cloning the repo and entering the directory):

``` shell
make build
sudo make install
```

This will build a portable version of the binary and then link it to `/usr/bin`
under the name `rsummarize`. If you'd rather build the version for your computer
and put it somewhere else (without a symlink in this case), run something like:

``` shell
cargo build -p summarize-bin --release
sudo cp target/release/summarize-bin /usr/bin/summarize
```

# Usage

Currently, `summarize` only works for output files produced by the Fortran
version of SPECTRO and the JSON output produced by the Rust version of
[spectro](https://github.com/ntBre/spectro) and
[pbqff](https://github.com/ntBre/pbqff), which uses that version internally.

To produce nicely-formatted text output, simply call `summarize` with the name
of the input file or input files:

``` shell
summarize testfiles/spectro.out
```

To list the options and available output formats, use the `--help` or `-h` flag.
One of the most useful of these is the `--tex` or `-t` flag, which outputs the
tables of data in a format directly usable in a paper.

