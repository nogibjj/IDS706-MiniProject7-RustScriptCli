[![Clippy](https://github.com/nogibjj/IDS706-MiniProject7-RustScriptCli/actions/workflows/lint.yml/badge.svg)](https://github.com/nogibjj/IDS706-MiniProject7-RustScriptCli/actions/workflows/lint.yml)
[![Tests](https://github.com/nogibjj/IDS706-MiniProject7-RustScriptCli/workflows/tests.yml/badge.svg)](https://github.com/nogibjj/IDS706-MiniProject7-RustScriptCli/actions/workflows/tests.yml)


# Rust Data Engineering

## Goal

> Duke University IDS 706 Weekly Mini Project 7


### Lab:  Modifying a Rust Command-Line Tool

In this lab you will gain experience extending an existing Rust project by forking and modifying a simple command-line tool.

**Steps**
4. Make a small modification to the tool such as:

   - Adding a new command line argument
    
   - Supporting additional input file formats
    
   - Adding more processing logic
    
   - Changing output formatting

5. Run `cargo build` to compile your changes  

6. Run `cargo run` to test your modified tool

7. Commit your changes and push to your forked repository


### Technical Notes

## Makefile

Each subdirectory project uses this style to make it easy to test and run

```
format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

run:
	cargo run 

all: format lint test run
```


## References

1. https://github.com/nogibjj/rust-data-engineering

* [Rust Collections](https://doc.rust-lang.org/std/collections/index.html)
* [GitHub Copilot CLI](https://www.npmjs.com/package/@githubnext/github-copilot-cli)
* [Rust Fundamentals](https://github.com/alfredodeza/rust-fundamentals)
* [Rust Tutorial](https://nogibjj.github.io/rust-tutorial/)
* [Rust MLOps Template](https://github.com/nogibjj/mlops-template)
