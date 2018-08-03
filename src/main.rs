extern crate clap;
extern crate mockery;

use mockery::{datatypes, model, generation, cli};

fn main() {
    let args = cli::get_args_from_stdin();

    let model_map = model::io::read_from_spec(&args.models_file_path).unwrap();
    let generation_spec = generation::io::generation_from_file(&args.gen_spec_path).unwrap();

    let output = generation_spec.generate_models(&model_map);

    generation_spec.generate_output_files(&args.output_path, &model_map).unwrap();

//    generation::io::write_models_to_file(&args.output_path, output).unwrap();
}

