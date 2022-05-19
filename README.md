# Programming Bitcoin Workshop

This repository contains the progress we made during an internal workshop we held at [LambdaClass](https://lambdaclass.com/). We went through _[Programming Bitcoin](https://www.oreilly.com/library/view/programming-bitcoin/9781492031482/)_ by Jimmy Song and implemented it's exercises in the Rust programming language.

## Lessons Learned

### Chapter 1 - Finite Fields

During the first round of the workshop, we:

- Implemented `struct FieldElement`, which models an element of a finite field.
- Implemented the `Add`, `Sub`, `Mul`, `Div` traits.
- Implemented a `pow` function.
- Wrote unit tests for the struct.

## Running Tests

To run tests, run the following command

```bash
cargo test
```

## Authors

- Federica Moletta, [@fmoletta](https://www.github.com/fmoletta).
- Milton Scuderi, [@mmsc2](https://www.github.com/mmsc2).
- Joaquín Pérez Centeno, [@jpcenteno](https://www.github.com/jpcenteno).
