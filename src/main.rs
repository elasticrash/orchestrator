use rand::Rng;

#[derive(Debug)]
struct Error {
    message: String,
}

#[derive(Debug)]
struct State {
    outcome: bool,
}

trait Chain {
    fn create<'a>(
        self,
        col: &'a std::vec::Vec<(fn(State) -> std::result::Result<State, Error>, &str)>,
    ) -> std::vec::Vec<&'a fn(State) -> std::result::Result<State, Error>>;
}

impl Chain for Vec<&str> {
    fn create<'a>(
        self,
        col: &'a std::vec::Vec<(fn(State) -> std::result::Result<State, Error>, &str)>,
    ) -> std::vec::Vec<&'a fn(State) -> std::result::Result<State, Error>> {
        self.iter()
            .map(|name| {
                let (g, _m): &(fn(state: State) -> Result<State, Error>, &str) =
                    col.iter().filter(|(_f, n)| n == name).next().unwrap();
                g
            })
            .collect()
    }
}

fn main() {
    let mut fun_collection: Vec<(fn(state: State) -> Result<State, Error>, &str)> = vec![];
    register(a, "a", &mut fun_collection);
    register(b, "b", &mut fun_collection);
    register(c, "c", &mut fun_collection);

    let chain = vec!["a", "b", "c"].create(&fun_collection);

    let x = chain.iter().fold(State { outcome: true }, |output, next| {
        next(output).unwrap()
    });
    println!("{:?}", x);
}

fn register<'a>(
    f: fn(state: State) -> Result<State, Error>,
    name: &'a str,
    fun_collection: &mut std::vec::Vec<(fn(State) -> std::result::Result<State, Error>, &'a str)>,
) {
    fun_collection.push((f, name))
}

fn a(c: State) -> Result<State, Error> {
    if c.outcome == false {
        Ok(c)
    } else {
        let mut rng = rand::thread_rng();
        let y: bool = rng.gen();
        Ok(State { outcome: y })
    }
}

fn b(c: State) -> Result<State, Error> {
    if c.outcome == false {
        Ok(c)
    } else {
        let mut rng = rand::thread_rng();
        let y: bool = rng.gen();
        Ok(State { outcome: y })
    }
}

fn c(c: State) -> Result<State, Error> {
    if c.outcome == false {
        Ok(c)
    } else {
        let mut rng = rand::thread_rng();
        let y: bool = rng.gen();
        Ok(State { outcome: y })
    }
}
