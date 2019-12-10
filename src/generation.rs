use crate::model::ModelMap;

use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap as StdHashMap;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GenerationSpecification {
	models: StdHashMap<String, usize>,
}

impl GenerationSpecification {
	pub fn generate_models(&self, models: &ModelMap) -> Vec<String> {
		use serde_json::to_string;
		let size = self.models.iter().fold(0, |t, (_, v)| t + v);
		let mut output = Vec::with_capacity(size);
		self.models.iter().for_each(|(k, v)| {
			let def = match models.get_model(k.to_string()) {
				Some(m) => m,
				None => return,
			};
			for _ in 0..*v {
				output.push(to_string(&def.generate_data()).unwrap())
			}
		});

		output
	}
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum OutputType {
	JSON,
	CSV,
}

impl OutputType {
	pub fn as_extension(&self) -> &'static str {
		use self::OutputType::*;
		match self {
			JSON => "json",
			CSV => "csv",
		}
	}
}

/// Contains all of the IO operations for output generation
pub mod io {
	use super::*;

	use crate::datatypes::RandomData;
	use crate::generation::{GenerationSpecification, OutputType};
	use crate::model::{Model, ModelMap};

	use std::convert::AsRef;
	use std::fs::{create_dir_all, read_to_string, File};
	use std::io::prelude::*;
	use std::io::{Error, ErrorKind, Result, SeekFrom};
	use std::iter::FromIterator;

	use csv::Writer as Csv;
	use serde_json::{from_str, to_string};

	use rayon::iter::IntoParallelRefIterator;
	use rayon::iter::ParallelIterator;
	use rayon_hash::HashMap;

	use std::sync::{Arc, Mutex};

	type SyncedVec<T> = Arc<Mutex<Vec<T>>>;

	impl GenerationSpecification {
		pub fn generate_output_files<P: AsRef<Path>>(
			&self,
			path: P,
			models: &ModelMap,
			out_type: OutputType,
		) -> Result<()> {
			create_dir_all(&path)?;

			let path_stub: String = path
				.as_ref()
				.to_str()
				.map_or(Err(Error::from(ErrorKind::NotFound)), |s| {
					Ok(String::from(s))
				})?;

			let mut ref_types: SyncedVec<(String, (String, RandomData))> =
				Arc::new(Mutex::new(Vec::new()));

			let par_map: HashMap<&String, &Model> =
				HashMap::from_iter(models.get_models_ref().iter());
			let err_list: Vec<Result<()>> = par_map
				.par_iter()
				.filter(|(key, _)| self.models.contains_key(**key))
				.map(|(type_name, model): (&&String, &&Model)| {
					let path = create_path(path_stub.to_string(), type_name.to_string(), out_type);
					let mut file = File::create(path)?;

					match out_type {
						OutputType::JSON => {
							file.write("[\n".as_ref())?;

							let quantity = self.models.get(*type_name).unwrap();
							for _ in 0..*quantity {
								file.write(to_string(&model.generate_data()).unwrap().as_ref())?;
								file.write(",\n".as_ref())?;
							}
							if quantity > &0usize {
								file.seek(SeekFrom::Current(-(",\n".as_bytes().len() as i64)))?;
							}

							file.write("\n]".as_ref())?;
						}
						OutputType::CSV => {
							let ordering = models.get_serialize_ref().get(*type_name);
							let mut writer = Csv::from_writer(file);
							let quantity = self.models.get(*type_name).unwrap();
							for _ in 0..*quantity {
								let data_set = model.generate_data();
								let mut row: Vec<String> = Vec::new();
								if let Some(order) = ordering {
									for key in order.iter() {
										row.push(
											data_set
												.get(key)
												.unwrap_or(&String::from("null"))
												.to_string(),
										);
									}
								} else {
									data_set.values().for_each(|v| row.push(v.clone()));
								}
								writer.write_record(&row)?;
							}
						}
					}

					let model_refs = model.get_reference_types();
					if model_refs.len() > 0 {
						let mut ref_types_vec = ref_types.lock().unwrap();
						for mr in model_refs {
							ref_types_vec.push((type_name.to_string(), mr));
						}
					}

					Ok(())
				})
				.collect();

			let required_refs = ref_types.lock().unwrap();

			for (model_name, (field_name, reference)) in required_refs.iter() {
				println!("{}, {}, {:?}", model_name, field_name, reference);
			}

			for i in err_list {
				i?;
			}

			Ok(())
		}
	}

	pub fn generation_from_file<P>(path: P) -> Result<GenerationSpecification>
	where
		P: AsRef<Path>,
	{
		let file = read_to_string(path)?;
		from_str(&file).map_err(|e| Error::from(e))
	}

	pub fn write_models_to_file<P>(path: P, models: Vec<String>) -> Result<()>
	where
		P: AsRef<Path>,
	{
		let mut file = File::create(&path)?;
		let mut flush_count = 0;
		file.write("[\n".as_ref())?;
		for model in models {
			file.write(model.as_ref())?;
			file.write("\n".as_ref())?;
			flush_count = flush_count + 1;
			if flush_count >= 20 {
				file.flush()?;
				flush_count = 0;
			}
		}
		file.write("]".as_ref())?;
		Ok(())
	}
}

fn create_path(prefix: String, file_name: String, output: OutputType) -> PathBuf {
	let mut path = PathBuf::from(prefix);
	path.push(file_name);
	path.with_extension(output.as_extension())
}
