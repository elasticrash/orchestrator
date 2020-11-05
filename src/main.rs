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
    fn execute(self) -> State;
}

impl<'a> Orchestrate for Vec<&'a fn(State) -> Result<State, Error>> {
    fn execute(self) -> State {
        self.iter().fold(
            State {
                proceed: true,
                outcome: 6.,
            },
            |output, next| next(output).unwrap(),
        )
    }
}

fn main() {
    let mut fun_collection: Vec<(fn(state: State) -> Result<State, Error>, &str)> = vec![];
    register(a, "pow2", &mut fun_collection);
    register(b, "pow3", &mut fun_collection);
    register(c, "sqrt", &mut fun_collection);

    let result = vec!["pow2", "pow3", "sqrt"]
        .create(&fun_collection)
        .execute();

    println!("{:?}", result);

    let result = vec!["pow3", "pow3", "sqrt", "sqrt"]
        .create(&fun_collection)
        .execute();

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
    if c.proceed == false {
        Ok(c)
    } else {
        let mut rng = rand::thread_rng();
        let y: bool = rng.gen();
        Ok(State {
            proceed: y,
            outcome: c.outcome.powf(2.0),
        })
    }
}

fn b(c: State) -> Result<State, Error> {
    if c.proceed == false {
        Ok(c)
    } else {
        let mut rng = rand::thread_rng();
        let y: bool = rng.gen();
        Ok(State {
            proceed: y,
            outcome: c.outcome.powf(3.0),
        })
    }
}

fn c(c: State) -> Result<State, Error> {
    if c.proceed == false {
        Ok(c)
    } else {
        let mut rng = rand::thread_rng();
        let y: bool = rng.gen();
        Ok(State {
            proceed: y,
            outcome: c.outcome.sqrt(),
        })
    }
}
