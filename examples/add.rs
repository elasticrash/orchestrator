extern crate orchestrator;
use orchestrator::{state_function, Chain, Error, Orchestrate, Register, Registry, State};

fn main() {
    let mut registry = Registry::new();
    let a: fn(State<String>) -> Result<State<String>, Error> = state_function!(add, String);

    registry.register(a, "add".to_string());

    let result = vec!["add", "add", "add"]
        .create(&registry.di_ref)
        .execute(State {
            proceed: true,
            outcome: Some("test".to_string()),
            stage: Vec::<bool>::new(),
        });

    println!("{:?}", result);
    assert_eq!(result.outcome.unwrap(), "testtesttesttesttesttesttesttest")
}

fn add(n: String) -> Option<String> {
    Some(format!("{}{}", n, n))
}
