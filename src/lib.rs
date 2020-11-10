use std::result::Result;
use std::vec::Vec;

#[macro_export]
macro_rules! state_function {
    ( $func:expr, $calc:ty) => {{
        pub fn state_fn(c: State<$calc>) -> Result<State<$calc>, Error> {
            let stage: Vec<bool> = c.stage.to_vec();
            if c.proceed == false {
                Ok(State {
                    proceed: false,
                    outcome: c.outcome,
                    stage: stage,
                })
            } else {
                let (result, proceed) = match $func(c.outcome.unwrap()) {
                    Some(_d) => (Some(_d), true),
                    _ => (None, true),
                };

                Ok(State {
                    proceed: proceed,
                    outcome: Some(result.unwrap()),
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

#[derive(Debug)]
pub struct State<T> {
    pub proceed: bool,
    pub outcome: Option<T>,
    pub stage: Vec<bool>,
}

pub trait Chain<T> {
    fn create<'a>(
        self,
        col: &'a Vec<(fn(State<T>) -> Result<State<T>, Error>, String)>,
    ) -> Vec<&'a fn(State<T>) -> Result<State<T>, Error>>;
}

impl<T> Chain<T> for Vec<&str> {
    fn create<'a>(
        self,
        col: &'a Vec<(fn(State<T>) -> Result<State<T>, Error>, String)>,
    ) -> Vec<&'a fn(State<T>) -> Result<State<T>, Error>> {
        self.iter()
            .map(|name| {
                let (g, _m): &(fn(state: State<T>) -> Result<State<T>, Error>, String) =
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
pub struct Registry<T> {
    pub di_ref: Vec<(fn(State<T>) -> Result<State<T>, Error>, String)>,
}

pub trait Register<T> {
    fn register(&mut self, f: fn(state: State<T>) -> Result<State<T>, Error>, name: String);
    fn new() -> Self;
}

impl<T> Register<T> for Registry<T> {
    fn register(&mut self, f: fn(state: State<T>) -> Result<State<T>, Error>, name: String) {
        self.di_ref.push((f, name));
    }
    fn new() -> Self {
        Self { di_ref: vec![] }
    }
}

pub trait Orchestrate<T> {
    fn execute(self, state: State<T>) -> State<T>;
}

impl<'a, T> Orchestrate<T> for Vec<&'a fn(State<T>) -> Result<State<T>, Error>> {
    fn execute(self, state: State<T>) -> State<T> {
        self.iter().enumerate().fold(state, |output, (i, func)| {
            let new_state = State {
                proceed: output.proceed,
                outcome: output.outcome,
                stage: output.stage.to_vec(),
            };
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
