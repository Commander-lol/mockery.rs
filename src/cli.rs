use clap::{Arg, App};
use std::path::PathBuf;
use std::default::Default;

use generation::OutputType;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CliArgs {
    pub models_file_path: PathBuf,
    pub gen_spec_path: PathBuf,
    pub output_path: PathBuf,
    pub output_type: OutputType,
}

impl <'s>From<&'s str> for OutputType {
    fn from(s: &'s str) -> Self {
        match s {
            "csv" => OutputType::CSV,
            "json" => OutputType::JSON,
            _ => OutputType::CSV,
        }
    }
}

impl Default for CliArgs {
    fn default() -> Self {
        CliArgs {
            models_file_path: PathBuf::default(),
            gen_spec_path: PathBuf::default(),
            output_path: PathBuf::default(),
            output_type: OutputType::CSV,
        }
    }
}

pub fn get_args_from_stdin() -> CliArgs {
    let matches = App::new("mockery.rs")
        .version("0.1.0")
        .author("Louis Capitanchik <louis.capitanchik@launchbase.solutions>")
        .about("Generate model data that can be inserted into a database")
        .after_help("To view help, use -h. For long form help, use --help")
        .arg(Arg::with_name("spec")
            .short("s")
            .long("spec")
            .help("Sets the spec file to use")
            .value_name("SPEC_PATH")
            .long_help("Sets the spec file to use. By default, mockery will look for a 'spec.json' file in CWD, and will error if it can not be found")
            .required(false))
        .arg(Arg::with_name("type")
            .short("t")
            .long("type")
            .help("Sets the output type")
            .value_name("OUTPUT_TYPE")
            .possible_value("csv")
            .possible_value("json")
            .long_help("Sets the output type. This value defaults to CSV for higher compatibility and throughput")
            .required(false))
        .arg(Arg::with_name("INPUT")
            .help("Sets the input file to use")
            .long_help("Sets the input file to use. The input file determines the cardinality of models to create, as well as any additional constraints that might be applied")
            .required(true)
            .index(1))
        .arg(Arg::with_name("OUTPUT")
            .help("Sets the output path. Must be a folder")
            .long_help("Sets the output path. Must be a folder; if it does not exist, it will be created. Files corresponding to the input model names will be created inside this folder")
            .required(true)
            .index(2))
        .get_matches();

    CliArgs {
        models_file_path: matches.value_of("spec")
            .map(|s| PathBuf::from(s))
            .unwrap_or_else(|| PathBuf::from("spec.json")),
        gen_spec_path: matches.value_of("INPUT")
            .map(|s| PathBuf::from(s))
            .unwrap(),
        output_path: matches.value_of("OUTPUT")
            .map(|s| PathBuf::from(s))
            .unwrap(),
        output_type: matches.value_of("type")
            .map(|s| OutputType::from(s))
            .unwrap_or(OutputType::CSV),
    }
}
