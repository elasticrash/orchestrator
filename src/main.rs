use rand::Rng;
use std::result::Result;
use std::vec::Vec;

macro_rules! state_function {
    ( $func:expr ) => {{
        pub fn state_fn(c: State) -> Result<State, Error> {
            let stage: Vec<bool> = c.stage.to_vec();
            if c.proceed == false {
                Ok(State {
                    proceed: false,
                    outcome: c.outcome,
                    stage: stage,
                })
            } else {
                let mut rng = rand::thread_rng();
                let y: bool = rng.gen();
                Ok(State {
                    proceed: y,
                    outcome: $func(c.outcome),
                    stage: stage,
                })
            }
        }

        state_fn
    }};
}

#[derive(Debug)]
pub struct Error {
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct State {
    pub proceed: bool,
    pub outcome: f32,
    pub stage: Vec<bool>,
}

trait Chain {
    fn create<'a>(
        self,
        col: &'a Vec<(fn(State) -> Result<State, Error>, String)>,
    ) -> Vec<&'a fn(State) -> Result<State, Error>>;
}

impl Chain for Vec<&str> {
    fn create<'a>(
        self,
        col: &'a Vec<(fn(State) -> Result<State, Error>, String)>,
    ) -> Vec<&'a fn(State) -> Result<State, Error>> {
        self.iter()
            .map(|name| {
                let (g, _m): &(fn(state: State) -> Result<State, Error>, String) =
                    match col.iter().filter(|(_f, n)| n == name).next() {
                        Some(res) => res,
                        None => panic!(format!("no function found with name {} exiting", name)),
                    };
                g
            })
            .collect()
    }
}

#[derive(Debug)]
struct Registry {
    pub di_ref: Vec<(fn(State) -> Result<State, Error>, String)>,
}

trait Register {
    fn register(&mut self, f: fn(state: State) -> Result<State, Error>, name: String);
    fn new() -> Self;
}

impl Register for Registry {
    fn register(&mut self, f: fn(state: State) -> Result<State, Error>, name: String) {
        self.di_ref.push((f, name));
    }
    fn new() -> Self {
        Self { di_ref: vec![] }
    }
}

trait Orchestrate {
    fn execute(self, state: State) -> State;
}

impl<'a> Orchestrate for Vec<&'a fn(State) -> Result<State, Error>> {
    fn execute(self, state: State) -> State {
        self.iter().enumerate().fold(state, |output, (i, func)| {
            let new_state = output.clone();
            if new_state.stage.len() > i {
                if new_state.stage[i] {
                    return new_state;
                } else {
                    let mut next_state = func(new_state).unwrap();
                    next_state.stage[i] = next_state.proceed;
                    return next_state;
                }
            }
            let mut next_state = func(new_state).unwrap();
            next_state.stage.push(next_state.proceed);
            return next_state;
        })
    }
}

fn main() {
    let mut registry = Registry::new();
    let a: fn(State) -> Result<State, Error> = state_function!(pow2);
    let b: fn(State) -> Result<State, Error> = state_function!(pow3);
    let c: fn(State) -> Result<State, Error> = state_function!(sqrt);

    registry.register(a, "pow2".to_string());
    registry.register(b, "pow3".to_string());
    registry.register(c, "sqrt".to_string());

    let result = vec!["pow2", "pow3", "sqrt"]
        .create(&registry.di_ref)
        .execute(State {
            proceed: true,
            outcome: 6.,
            stage: Vec::<bool>::new(),
        });

    println!("{:?}", result);

    let result = vec!["pow3", "pow3", "sqrt", "sqrt"]
        .create(&registry.di_ref)
        .execute(State {
            proceed: true,
            outcome: 6.,
            stage: vec![true, true, false, false],
        });

    println!("{:?}", result);
}

fn pow2(n: f32) -> f32 {
    n.powf(2.0)
}

fn pow3(n: f32) -> f32 {
    n.powf(3.0)
}

fn sqrt(n: f32) -> f32 {
    n.sqrt()
}
