# Orchestration

The orchestration library is designed to streamline and manage complex workflows by providing a framework for orchestrating and sequencing functions in a controlled manner. It allows developers to define a series of state functions and execute them in a predefined order, passing data between each step. With the ability to handle various types, the library offers flexibility in processing different data structures.

### Improvements and changes:

* Enhanced type handling capabilities to accommodate a wider range of types.
* String Concatenation Example: The updated documentation now includes an example showcasing string concatenation. This addition helps users understand how to perform string operations within their orchestration workflows.
* Improved Calculation Example: The calculation example has been updated to demonstrate the improved capabilities of the library. Users can now leverage the library's features for performing calculations more effectively.

## Setup

To define an orchestration function, you can utilize the state_function! macro. Here's an example:


``` rust
 let fn1: fn(State<f32>) -> Result<State<f32>, Error> =
        state_function!(pow2, f32);
```

A state is represented by the following structure:

``` rust
pub struct State<T> {
    pub proceed: bool,
    pub outcome: Option<T>,
    pub stage: Vec<bool>,
}
```

In some cases, you can directly utilize the orchestration by employing the following approach:

``` rust
  let result = vec![fn1, fn2, fn3]
        .execute(State {
            proceed: true,
            outcome: Some(6.),
            stage: Vec::<bool>::new(),
        });
```

Alternatively, you can use the registration trait to assign string names to the orchestration functions. This approach proves useful when configuring function sequences more generically:

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

By assigning values to the stage, as shown in the example below, you can bypass specific steps in the sequence. Marking a step as true allows it to be skipped:


```rust
            stage: vec![true, true, false, false],
``` 

For a more intricate example, please refer to the following link: 
https://github.com/elasticrash/keyboard/blob/master/lib/src/geometry/exported_geometry.rs
