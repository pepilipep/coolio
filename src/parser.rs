use clap::{app_from_crate, arg, ArgMatches};

pub struct Parser {
    matches: ArgMatches,
}

impl Parser {
    pub fn new() -> Self {
        let matches = app_from_crate!()
            .arg(arg!(--test <VALUE>))
            .arg(arg!(--test2 <VALUE>))
            .get_matches();
        Parser { matches }
    }

    pub fn parse(&self) -> () {
        println!(
            "test - {:?}, test2 - {:?}",
            self.matches.value_of("test").expect("required"),
            self.matches.value_of("test2").expect("required")
        );
    }
}
