use crate::datatypes::RandomData;
use std::string::ToString;
use std::iter::Iterator;
use failure::Fail;

use serde_derive::{Deserialize, Serialize};

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
    use super::Specification;
    use crate::generator::from_spec;

    use std::io::Result;
    use std::fs::read_to_string;
    use std::path::{Path, PathBuf};
    use std::convert::AsRef;
    use failure::{Fail};

    use serde_derive::{Deserialize, Serialize};
    use serde_json::{from_str, error::Category};

    pub type SpecResult<Success> = std::result::Result<Success, SpecError>;

    #[derive(Debug, Fail)]
    pub enum SpecError {
        #[fail(display = "Could not find the spec file: {}", 0)]
        MissingFile(String),
        #[fail(display = "Failed for unhandled reason: {}", inner)]
        IOError { inner: std::io::Error },
        #[fail(display = "The provided spec path could not be correctly converted")]
        BadPath,
        #[fail(display = "The specification is not valid JSON")]
        BadFormat,
        #[fail(display = "The specification contained one or more invalid definitions")]
        BadData
    }

    fn pathable_to_string<P: AsRef<Path>>(path: &P) -> SpecResult<String> {
        return match path.as_ref().to_str() {
            Some(string) => Ok(String::from(string)),
            None => Err(SpecError::BadPath)
        }
    }

    pub fn read_spec<P: AsRef<Path>>(path: P) -> SpecResult<Specification> {
        let content = match read_to_string(&path) {
            Ok(content) => content,
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => {
                    return Err(SpecError::MissingFile(pathable_to_string(&path)?))
                }
                _ => return Err(SpecError::IOError { inner: e })
            }
        };

        match from_str(&content) {
            Ok(spec) => Ok(spec),
            Err(e) => match e.classify() {
                Category::Eof |
                Category::Syntax => Err(SpecError::BadFormat),
                Category::Data => Err(SpecError::BadData),
                Category::Io => Err(SpecError::IOError { inner: e.into() }),
            }
        }
    }
}
