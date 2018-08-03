use std::collections::HashMap as StdHashMap;
use model::ModelMap;
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GenerationSpecification {
    models: StdHashMap<String, usize>,
}

impl GenerationSpecification {
    pub fn generate_models(&self, models: &ModelMap) -> Vec<String> {
        use serde_json::to_string;
        let size = self.models.iter().fold(0, |t, (k, v)| t + v);
        let mut output = Vec::with_capacity(size);
        self.models.iter().for_each(|(k, v)| {
            let def = match models.get_model(k.to_string()) {
                Some(m) => m,
                None => return,
            };
            for i in 0..*v {
                output.push(to_string(&def.generate_data()).unwrap())
            }
        });

        output
    }
}

pub mod io {
    use std::io::prelude::*;
    use std::io::{Result, Error, ErrorKind, SeekFrom};
    use std::fs::{File, read_to_string, create_dir_all};
    use std::path::{Path, PathBuf};
    use std::convert::AsRef;
    use std::iter::FromIterator;

    use serde_json::{from_str, to_string};

    use model::{ModelMap, Model};
    use generation::GenerationSpecification;
    use rayon_hash::HashMap;
    use rayon::iter::IntoParallelRefIterator;
    use rayon::iter::ParallelIterator;

    impl GenerationSpecification {
        pub fn generate_output_files<P: AsRef<Path>>(&self, path: P, models: &ModelMap) -> Result<()> {
            create_dir_all(&path)?;

            let path_stub: String = path.as_ref()
                .to_str()
                .map_or(Err(Error::from(ErrorKind::NotFound)), |s| Ok(String::from(s)))?;

            let mut par_map: HashMap<&String, &Model> = HashMap::from_iter(models.get_models_ref().iter());
            let err_list: Vec<Result<()>> = par_map.par_iter()
                .filter(|(key, _)| self.models.contains_key(**key))
                .map(|(type_name, model)| {
                    let mut path = PathBuf::from(&path_stub);
                    path.push(type_name);

                    let mut file = File::create(path.with_extension("json"))?;
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

                    Ok(())
                })
                .collect();

            for i in err_list {
                i?;
            }

            Ok(())
        }
    }

    pub fn generation_from_file<P>(path: P) -> Result<GenerationSpecification> where P: AsRef<Path> {
        let file = read_to_string(path)?;
        from_str(&file).map_err(|e| Error::from(e))
    }

    pub fn write_models_to_file<P>(path: P, models: Vec<String>) -> Result<()> where P: AsRef<Path> {
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
