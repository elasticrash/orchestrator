use rand::Rng;
use std::result::Result;
use std::vec::Vec;

#[derive(Debug)]
struct Error {
    message: String,
}

#[derive(Debug)]
struct State {
    proceed: bool,
    outcome: f32,
    stage: Vec<bool>,
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
        self.iter()
            .fold(state, |output, next| next(output).unwrap())
    }
}

fn main() {
    let mut registry = Registry::new();
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
            stage: Vec::<bool>::new(),
        });

    println!("{:?}", result);
}

fn a(c: State) -> Result<State, Error> {
    let mut stage: Vec<bool> = c.stage.to_vec();
    stage.push(c.proceed);
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
            outcome: c.outcome.powf(2.0),
            stage: stage,
        })
    }
}

fn b(c: State) -> Result<State, Error> {
    let mut stage: Vec<bool> = c.stage.to_vec();
    stage.push(c.proceed);
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
            outcome: c.outcome.powf(3.0),
            stage: stage,
        })
    }
}

fn c(c: State) -> Result<State, Error> {
    let mut stage: Vec<bool> = c.stage.to_vec();
    stage.push(c.proceed);
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
            outcome: c.outcome.sqrt(),
            stage: stage,
        })
    }
}
