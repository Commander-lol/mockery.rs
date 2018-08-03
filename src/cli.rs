use clap::{Arg, App};
use std::path::{Path, PathBuf};
use std::default::Default;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CliArgs {
    pub models_file_path: PathBuf,
    pub gen_spec_path: PathBuf,
    pub output_path: PathBuf,
}

impl Default for CliArgs {
    fn default() -> Self {
        CliArgs {
            models_file_path: PathBuf::default(),
            gen_spec_path: PathBuf::default(),
            output_path: PathBuf::default(),
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
        .arg(Arg::with_name("INPUT")
            .help("Sets the input file to use")
            .required(true)
            .index(1))
        .arg(Arg::with_name("OUTPUT")
            .help("Sets the output path. Setting to '-' will stream to stdout")
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
    }
}
