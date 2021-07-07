# moss-fmt

[![Build Status](https://travis-ci.com/CJohnson19/moss-fmt.svg?branch=master)](https://travis-ci.com/CJohnson19/moss-fmt)

## Description

moss-fmt is an executable tool that searches a directory of zips and extracts
only the specified files to a new directory, renaming them to identifiable names
in the process. The main reason being formatting files for
[Moss](https://theory.stanford.edu/~aiken/moss/).

## Installation

### Via Cargo (Recommended)

moss-fmt can be installed via [Cargo](https://doc.rust-lang.org/cargo/), which
comes with [Rustup](https://www.rust-lang.org/learn/get-started) via:

``` sh
cargo install moss-fmt
```

### Manually building

You may manually build the source code using:

``` sh
$ git clone https://github.com/CJohnson19/moss-fmt.git
$ cd moss-fmt
$ cargo build --release
```

The binary will then be in `target/release/moss-fmt`.

## Usage

``` sh
moss-fmt [FLAGS] [OPTIONS] --dir <dir> --filename <file>
```

### Flags

- `-h`, `--help`: print help information
- `-V`, `--version`: print version number
- `-v`, `--verbose`: print information about each operation performed

### Options

- `-d`, `--dir`: Relative path to a directory with zip files you want to search.
  Multiple directories may be provided.
- `-f`, `--filename`: Filename to search for in each zip. Multiple file names
  can be provided.
- `--filter-dir`: Folder to exclude from search in each zip. Defaults to
  `__MACOSX` and `node_modules`.
- `-o`, `--output`: Directory to store resulting files. Defaults to current
  working directory.


## Features

- Renaming of files to individually identifiable names. `index.js` turns into
  `johndoe-index.js`.
- No artifacts retained from the extraction process, only the files you want get
  removed.
- Filtering of file's parent folders, that means no `__MACOSX` or `node_modules`
  to deal with.
