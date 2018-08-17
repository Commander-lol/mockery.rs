use datatypes::RandomData;
use std::string::ToString;
use std::iter::Iterator;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Reference {
    path: String,
    property: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "value")]
pub enum DataType {
    RandomData(RandomData),
    List(Box<DataType>),
    Model(String),
    Reference {
        path: String,
        property: String,
    },
}

use std::collections::{HashMap, hash_map::Iter};
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Model {
    #[serde(flatten)]
    properties: HashMap<String, DataType>,
}

impl Model {
    pub fn type_iter(&self) -> Iter<String, DataType> {
        self.properties.iter()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Specification {
    serialize: HashMap<String, Vec<String>>,
    models: HashMap<String, Model>,
}

impl Specification {
    pub fn has_model<S: ToString>(&self, name: S) -> bool {
        self.models.contains_key(&name.to_string())
    }
    pub fn get_definition<S: ToString>(&self, name: S) -> &Model {
        &self.models.get(&name.to_string()).unwrap()
    }
    pub fn get_serialize_ref<S: ToString>(&self, name: S) -> Option<&Vec<String>> {
        self.serialize.get(&name.to_string())
    }
}


pub mod io {
    use std;
    use std::io::Result;
    use std::fs::read_to_string;
    use super::Specification;
    use std::path::{Path, PathBuf};
    use std::convert::AsRef;
    use serde_json::from_str;

    pub fn read_spec<P: AsRef<Path>>(path: P) -> Option<Specification> {
        let content = read_to_string(path).ok()?;
        Some(from_str(&content).unwrap())
    }
}
