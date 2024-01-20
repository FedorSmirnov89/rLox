# RLox - Parser + Interpreter for Lox in Rust

Code written when working through Robert Nystrom's [Crafting Interpreters](https://craftinginterpreters.com/).

## Running things

Build:

```
cargo b
```

Running as prompt:

```
cargo r -- prompt
```

Running as interpreter for a file:

```
cargo r -- file [file_path]
```

## Differences to the implementation in the book

- Variable assignment is a statement and not an expression