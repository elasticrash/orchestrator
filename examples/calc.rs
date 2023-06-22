extern crate orchestrator;
use orchestrator::{state_function, Chain, Error, Orchestrate, Register, Registry, State};

fn main() {
    let mut registry = Registry::new();
    let a: fn(State<f32>) -> Result<State<f32>, Error> = state_function!(pow2, f32);
    let b: fn(State<f32>) -> Result<State<f32>, Error> = state_function!(pow3, f32);
    let c: fn(State<f32>) -> Result<State<f32>, Error> = state_function!(sqrt, f32);

    registry.register(a, "pow2".to_string());
    registry.register(b, "pow3".to_string());
    registry.register(c, "sqrt".to_string());

    let result = vec!["pow2", "pow3", "sqrt"]
        .create(&registry.di_ref)
        .execute(State {
            proceed: true,
            outcome: Some(6.),
            stage: Vec::<bool>::new(),
        });

    println!("{:?}", result);
    assert_eq!(result.outcome.unwrap(), 216_f32);

    let result = vec!["pow3", "pow3", "sqrt", "sqrt"]
        .create(&registry.di_ref)
        .execute(State {
            proceed: true,
            outcome: Some(6.),
            stage: vec![true, true, false, false],
        });

    println!("{:?}", result);
    assert_eq!(result.outcome.unwrap(), 1.5650846);
}

fn pow2(n: f32) -> Option<f32> {
    Some(n.powf(2.0))
}

fn pow3(n: f32) -> Option<f32> {
    Some(n.powf(3.0))
}

fn sqrt(n: f32) -> Option<f32> {
    Some(n.sqrt())
}
