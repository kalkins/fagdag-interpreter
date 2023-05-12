# KWLang Interpreter

This is a parser and interpreter for a simple custom language, written in Rust.

The grammar is defined in `src/kwlang.pest`, parsed by `src/parser`, and executed by `src/vm`.

## Usage

```bash
cargo run -- tests/simple_return.kw
```