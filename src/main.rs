use failure::{Error, Fail};
use mockery::{cli, generation, generator, model};

fn main() {
	let args = cli::get_args_from_stdin();
	match process_args(args) {
		Ok(_) => (),
		Err(e) => eprintln!("{}", e),
	}
}

/// Wrap a string in a failure compatible container until refactoring is complete
#[derive(Debug, Fail)]
enum StringErrorCompat {
	#[fail(display = "{}", 0)]
	S(String),
}

fn process_args(args: cli::CliArgs) -> Result<(), Error> {
	use mockery::specification::{self, Specification};
	extern crate serde_json;

	let model_name = &args.model_name;
	let spec = specification::io::read_spec(&args.gen_spec_path)?;

	if spec.has_model(&model_name) {
		let data = generator::from_spec(model_name.clone(), spec.clone(), args.model_amount)
			.map_err(|e| StringErrorCompat::S(e))?;

		generator::write_output(
			&args.output_path,
			data,
			spec,
			args.output_type,
			args.pretty_print,
		);
	} else {
		println!(
			"No such model {} in {:?}",
			&model_name,
			&args.gen_spec_path.to_str()
		);
	}

	Ok(())
}
