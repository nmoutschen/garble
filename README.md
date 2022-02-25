# Data garbling crate

The purpose of this crate is to provide a way to slightly modify data in
controlled way for fault injection purposes.

## Example

```rust
use garble::{Garble, SimpleGarbler};

// Create a garbler with a 50% probability of garbling data
let mut garbler = SimpleGarbler::new(0.5);

// Garble some data
dbg!(true.garble(&mut garbler));
dbg!(128u64.garble(&mut garbler));
dbg!((3.5_f32).garble(&mut garbler));
```

## Derive macro

This crate provides a derive macro for garbling structs.

```rust
use garble::{Garble, SimpleGarbler};

#[derive(Debug, Garble)]
struct MyStruct {
   a: u32,
}

// Create a garbler with a 50% probability of garbling data
let mut garbler = SimpleGarbler::new(0.5);

// Garble some data
dbg!(MyStruct { a: 128 }.garble(&mut garbler));
```
