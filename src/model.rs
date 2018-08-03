use std::collections::HashMap;
use datatypes::RandomData;
use std::string::ToString;
use std::convert::AsRef;

type Map<T> = HashMap<String, T>;
type GeneratedProperties = Map<String>;
type GeneratedModels = Map<GeneratedProperties>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Model(HashMap<String, RandomData>);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelMap {
    models: HashMap<String, Model>,
}

impl Model {
    pub fn new() -> Self {
        Model(HashMap::new())
    }

    pub fn add_property<TS>(&mut self, name: TS, definition: RandomData) -> &mut Self where TS: ToString {
        self.0.insert(name.to_string(), definition);
        self
    }
    pub fn get_property(&self, name: String) -> Option<&RandomData> {
        self.0.get(&name)
    }
    pub fn generate_data(&self) -> GeneratedProperties {
        let mut generated = HashMap::with_capacity(self.0.len());
        for (name, generator) in &self.0 {
            generated.insert(name.clone(), generator.to_string());
        }

        generated
    }
}

pub mod io {
    use std::path::Path;
    use std::io::prelude::*;
    use std::io::{Result, Error};
    use std::fs::read_to_string;
    use model::ModelMap;
    use serde_json::{from_str, to_string};

    pub fn read_from_spec<P>(path: P) -> Result<ModelMap> where P: AsRef<Path> {
        let file = read_to_string(path.as_ref())?;
        from_str(&file).map_err(|e| Error::from(e))
    }
    pub fn write_to_spec<P>(model: ModelMap, path: P) -> Result<()> where P: AsRef<Path> {
        let output = to_string(&model);
        println!("{:?}", output);
        Ok(())
    }
}

impl ModelMap {
    pub fn new() -> Self {
        ModelMap {
            models: HashMap::new(),
        }
    }

    pub fn add_model<TS>(&mut self, name: TS, model: Model) -> &mut Self where TS: ToString {
        self.models.insert(name.to_string(), model);
        self
    }
    pub fn get_model(&self, name: String) -> Option<&Model> {
        self.models.get(&name)
    }
    pub fn generate_data(&self) -> GeneratedModels {
        let mut generated = HashMap::with_capacity(self.models.len());
        for (name, model) in &self.models {
            generated.insert(name.clone(), model.generate_data());
        }

        generated
    }
    pub fn get_models_ref(&self) -> &HashMap<String, Model> {
        &self.models
    }
}

#[test]
fn generate_random_data() {
    use model::Model;
    use datatypes::RandomData;

    let mut model = Model::new();

    model
        .add_property("name".to_owned(), RandomData::FullName)
        .add_property("email".to_owned(), RandomData::Email);

    println!("Random user: {:?}", model.generate_data());
}
