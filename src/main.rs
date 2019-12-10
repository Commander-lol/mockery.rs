use mockery::{model, generation, cli, generator};
use clap;

fn main() {
    let args = cli::get_args_from_stdin();
    process_args(args);
}

fn process_args(args: cli::CliArgs) {

    use mockery::specification::{Specification, self};
    extern crate serde_json;

    let model_name = &args.model_name;
    let spec = specification::io::read_spec(&args.gen_spec_path).unwrap();

    if spec.has_model(&model_name) {
        let data = generator::from_spec(model_name.clone(), spec.clone());

        match data {
            Ok(res) => {
                generator::write_output(&args.output_path, res, spec, args.output_type);
            },
            Err(e) => println!("Error: {}", e),
        }
    } else {
        println!("No such model {} in {:?}", &model_name, &args.gen_spec_path.to_str());
    }
}
