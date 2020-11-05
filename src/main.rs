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
        col: &'a Vec<(fn(State) -> Result<State, Error>, &str)>,
    ) -> Vec<&'a fn(State) -> Result<State, Error>>;
}

impl Chain for Vec<&str> {
    fn create<'a>(
        self,
        col: &'a Vec<(fn(State) -> Result<State, Error>, &str)>,
    ) -> Vec<&'a fn(State) -> Result<State, Error>> {
        self.iter()
            .map(|name| {
                let (g, _m): &(fn(state: State) -> Result<State, Error>, &str) =
                    match col.iter().filter(|(_f, n)| n == name).next() {
                        Some(res) => res,
                        None => panic!(format!("no function found with name {} exiting", name)),
                    };
                g
            })
            .collect()
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
    let mut fun_collection: Vec<(fn(state: State) -> Result<State, Error>, &str)> = vec![];
    register(a, "pow2", &mut fun_collection);
    register(b, "pow3", &mut fun_collection);
    register(c, "sqrt", &mut fun_collection);

    let result = vec!["pow2", "pow3", "sqrt"]
        .create(&fun_collection)
        .execute(State {
            proceed: true,
            outcome: 6.,
            stage: Vec::<bool>::new(),
        });

    println!("{:?}", result);

    let result = vec!["pow3", "pow3", "sqrt", "sqrt"]
        .create(&fun_collection)
        .execute(State {
            proceed: true,
            outcome: 6.,
            stage: Vec::<bool>::new(),
        });

    println!("{:?}", result);
}

fn register<'a>(
    f: fn(state: State) -> Result<State, Error>,
    name: &'a str,
    fun_collection: &mut Vec<(fn(State) -> Result<State, Error>, &'a str)>,
) {
    fun_collection.push((f, name))
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
