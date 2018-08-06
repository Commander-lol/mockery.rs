extern crate clap;
extern crate uuid;
#[macro_use] extern crate fake;
extern crate chrono;

extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate regex;

extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

extern crate rayon;
extern crate rayon_hash;
extern crate csv;

/// All routines related to the insertion of fake data into a Postgresql database are contained
/// within the **database** module
pub mod database;
/// **datatypes** contains objects relevant for the definition and generation of fake data. Both
/// the `RandomData` and `generate_fake_data` items are found within this module
pub mod datatypes;
pub mod model;
pub mod generation;
pub mod cli;