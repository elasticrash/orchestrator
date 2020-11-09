use std::result::Result;
use std::vec::Vec;

#[macro_export]
macro_rules! state_function {
    ( $func:expr, $calc:ty, $err:ty) => {{
        pub fn state_fn(c: State<$calc, $err>) -> Result<State<$calc, $err>, Error> {
            let stage: Vec<bool> = c.stage.to_vec();
            if c.proceed == false {
                Ok(State {
                    proceed: false,
                    outcome: c.outcome,
                    stage: stage,
                })
            } else {
                let (result, proceed) = match $func(c.outcome.unwrap()) {
                    Ok(_d) => (_d, true),
                    _ => (0., true),
                };

                Ok(State {
                    proceed: proceed,
                    outcome: Ok(result),
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
pub struct State<T, E> {
    pub proceed: bool,
    pub outcome: Result<T, E>,
    pub stage: Vec<bool>,
}

pub trait Chain<T, E> {
    fn create<'a>(
        self,
        col: &'a Vec<(fn(State<T, E>) -> Result<State<T, E>, Error>, String)>,
    ) -> Vec<&'a fn(State<T, E>) -> Result<State<T, E>, Error>>;
}

impl<T, E> Chain<T, E> for Vec<&str> {
    fn create<'a>(
        self,
        col: &'a Vec<(fn(State<T, E>) -> Result<State<T, E>, Error>, String)>,
    ) -> Vec<&'a fn(State<T, E>) -> Result<State<T, E>, Error>> {
        self.iter()
            .map(|name| {
                let (g, _m): &(fn(state: State<T, E>) -> Result<State<T, E>, Error>, String) =
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
pub struct Registry<T, E> {
    pub di_ref: Vec<(fn(State<T, E>) -> Result<State<T, E>, Error>, String)>,
}

pub trait Register<T, E> {
    fn register(&mut self, f: fn(state: State<T, E>) -> Result<State<T, E>, Error>, name: String);
    fn new() -> Self;
}

impl<T, E> Register<T, E> for Registry<T, E> {
    fn register(&mut self, f: fn(state: State<T, E>) -> Result<State<T, E>, Error>, name: String) {
        self.di_ref.push((f, name));
    }
    fn new() -> Self {
        Self { di_ref: vec![] }
    }
}

pub trait Orchestrate<T, E> {
    fn execute(self, state: State<T, E>) -> State<T, E>;
}

impl<'a, T, E> Orchestrate<T, E> for Vec<&'a fn(State<T, E>) -> Result<State<T, E>, Error>> {
    fn execute(self, state: State<T, E>) -> State<T, E> {
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
