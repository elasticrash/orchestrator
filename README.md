# Orchestration

This is a basic orchestration library written in Rust and still under development

### 0.2 version now should be able to handle more types

* breaking change, state functions need to return Option instead of Result
* added example for string concatenation
* updated example for calculations

### 0.1 version only worked with f32

## Setup

### You define an orchestration function by using `state_function!` macro

i.e.

``` rust
 let fn1: fn(State<f32>) -> Result<State<f32>, Error> =
        state_function!(pow2, f32);
```

A state is defined as follows

``` rust
pub struct State<T> {
    pub proceed: bool,
    pub outcome: Option<T>,
    pub stage: Vec<bool>,
}
```

You can use the orchestration directly by using (in some cases)

``` rust
  let result = vec![fn1, fn2, fn3]
        .execute(State {
            proceed: true,
            outcome: Some(6.),
            stage: Vec::<bool>::new(),
        });
```

Or by using the registration trait and assigning string names to the orchestration functions.
Which can be useful when you want to pass function sequences by configuration (more generic)

``` rust
    registry.register(fn1, "pow2".to_string());
    registry.register(fn2, "pow3".to_string());
    registry.register(fn3, "sqrt".to_string());

        let result = vec!["pow2", "pow3", "sqrt"]
        .create(&registry.di_ref)
        .execute(State {
            proceed: true,
            outcome: Some(6.),
            stage: Vec::<bool>::new(),
        });
```

Assigning values to the stage i.e.

```rust
            stage: vec![true, true, false, false],
``` 

allows you to bypass certain steps (marked as true, in the sequence) 


A more complicated example can be found here
https://github.com/elasticrash/keyboard/blob/master/lib/src/geometry/exported_geometry.rs
