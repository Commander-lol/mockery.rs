extern crate clap;
extern crate mockery;

use mockery::{model, generation, cli};

fn main() {
    let args = cli::get_args_from_stdin();
    process_args(args);
}

fn process_args(args: cli::CliArgs) {
    let model_map = model::io::read_from_spec(&args.models_file_path).unwrap();
    let generation_spec = generation::io::generation_from_file(&args.gen_spec_path).unwrap();

    generation_spec.generate_output_files(&args.output_path, &model_map, args.output_type).unwrap();
}
