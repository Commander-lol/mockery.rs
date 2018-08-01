extern crate clap;
extern crate uuid;
#[macro_use] extern crate fake;
extern crate chrono;

extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate regex;
#[macro_use] extern crate lazy_static;

extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

mod database;
mod factories;
mod datatypes;

fn main() {
    let matches = {
        use clap::{Arg, App, SubCommand};
        App::new("Super Agent Mockery")
            .version("0.1.0")
            .author("Louis Capitanchik <louis.capitanchik@launchbase.solutions>")
            .about("Generate model data that can be inserted into the Super Agent database")
            .arg(Arg::with_name("INPUT")
                .help("Sets the input file to use. It should define the models that can be generated")
                .required(true)
                .index(1))
            .get_matches()
    };

    println!("{}", datatypes::generate_fake_data(datatypes::RandomData::Company));

}

#[test]
fn test_fn() {
    fn main() {
        let r = regex::Regex::new(r#"ParentType\((\w+)\)"#).unwrap();
        let caps = r.captures("ParentType(foo)");
        if let Some(captures) = caps {
            println!("{:?}, {:?}", captures.get(0), captures.get(1));
        }
    }

    main()
}

