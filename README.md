### Getting Started

#### How it works

1. Write the [language grammar](./src/grammar)
2. Generate the language parser

```
cargo run --features=gen
```

3. Parse a human-readable expression into an [Abstract Syntax Tree](https://en.wikipedia.org/wiki/Abstract_syntax_tree)

#### Example

```
cargo run --example tester "<expression>"
cargo run --example tester "[0,1,2][0]"
```

### Problem
How do we differentiate between these two:
- a list with a single integer element: `[0]`
- a list access operator: `[1,2,3][0]`
