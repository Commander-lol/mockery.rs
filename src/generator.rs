use crate::generation::OutputType;
use crate::datatypes::RandomData;

use std::collections::HashMap;
use specification::{Specification, Model, DataType as DT};
use std::borrow::Borrow;
use std::str::FromStr;
use std::rc::Rc;
use std::path::PathBuf;

// ------- TODO: Check these -------
use std::fs::{File, read_to_string, create_dir_all};
use std::convert::AsRef;
use std::iter::FromIterator;

use serde_json::{from_str, to_string};
use csv::Writer as Csv;

// ---------------------------------

pub type ModelDataMap = HashMap<String, Vec<HashMap<String, String>>>;

fn get_model_children(model: &Model) -> Vec<String> {
    model.type_iter()
        .filter_map(|(key, data_type)| {
            if let DT::Model(def) = data_type {
                Some(def.clone())
            } else if let DT::List(nested) = data_type {
                match nested.borrow() {
                    DT::Model(def) => Some(def.clone()),
                    _ => None,
                }
            } else {
                None
            }
        })
        .collect()
}

type BoolResult = Result<(), String>;
fn validate_dependencies(deps: &Vec<String>, spec: &Specification) -> BoolResult {
    let types_valid: Vec<BoolResult> = deps.iter()
        .map(|data_type|
            if spec.has_model(data_type) {
                Ok(())
            } else {
                Err(format!("No such type {}", data_type))
            }
        )
        .collect();

    for e in types_valid {
        e?;
    }

    Ok(())
}

#[derive(Clone, Debug)]
struct GenData {
    model: Model,
    data: HashMap<String, String>,
}

#[derive(Clone, Debug)]
struct GenContext {
    pub parent_context: Option<Box<GenContext>>,
    pub parent_model: Option<GenData>,
    pub models: ModelDataMap,
}

enum RefType {
    Parent,
}

impl FromStr for RefType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        match s {
            "^" => Ok(RefType::Parent),
            _ => Err("Invalid ref string"),
        }
    }
}

impl GenContext {
    pub fn add_model_data(&mut self, data_type: String, data_values: HashMap<String, String>) {
        let list = self.models
            .entry(data_type)
            .or_insert(Vec::new());
        list.push(data_values);
    }
    pub fn merge_model_data(&mut self, other_data: &mut ModelDataMap) {
        other_data.iter_mut().for_each(|(data_type, values)| {
            let list = self.models.entry(data_type.clone()).or_insert(Vec::new());
            list.append(values);
        });
    }
    pub fn fetch_ref_path(&self, parts: Vec<String>) -> Option<HashMap<String, String>> {
        if parts.len() == 0 {
            None
        } else if parts.len() == 1 {
            let final_ref = parts.get(0).unwrap();
            match RefType::from_str(final_ref).ok()? {
                RefType::Parent => {
                    if let Some(parent_model) = &self.parent_model {
                        Some(parent_model.data.clone())
                    } else {
                        None
                    }
                },
                _ => None,
            }
        } else {
            let next_ref = parts.get(0).unwrap();
            let rest: Vec<String> = parts.iter().skip(1).map(|s| s.clone()).collect();

            match RefType::from_str(next_ref).ok()? {
                RefType::Parent => {
                    if let Some(parent_ctx) = &self.parent_context {
                        let parent: &GenContext = parent_ctx.borrow();
                        parent.fetch_ref_path(rest)
                    } else {
                        None
                    }
                },
                _ => None,
            }
        }
    }
}

pub fn from_spec(model_name: String, spec: Specification) -> Result<ModelDataMap, String> {
    let initial_model = spec.get_definition(&model_name);
    let deps = get_model_children(initial_model);

    validate_dependencies(&deps, &spec)?;

    let mut initial_context = GenContext {
        parent_context: None,
        parent_model: None,
        models: HashMap::new(),
    };

    generate_model_data(model_name.clone(), initial_model, &mut initial_context, &spec);

    Ok(initial_context.models)
}

fn generate_model_data(model_type: String, model: &Model, ctx: &mut GenContext, spec: &Specification) {
    let mut model_data: HashMap<String, String> = HashMap::new();
    let mut child_models: Vec<(String, DT)> = Vec::new();

    model.type_iter().for_each(|(property, data_type)| {
        match data_type {
            DT::RandomData(random_data) => {
                let data = random_data.to_string();
                &model_data.insert(property.clone(), data);
            },
            DT::Model(_) => {
                child_models.push((property.clone(), data_type.clone()));
            },
            DT::List(_) => {
                child_models.push((property.clone(), data_type.clone()));
            },
            DT::Reference { ref path, property: ref_prop } => {
                let parts = path.split("~").map(String::from).collect();
                let ref_model_data = ctx.fetch_ref_path(parts);
                match ref_model_data {
                    Some(data_set) => {
                        let data = data_set.get(ref_prop).unwrap();
                        &model_data.insert(property.clone(), data.clone());
                    },
                    None => {}
                }
            },
            _ => {}
        }
    });

    ctx.add_model_data(model_type, model_data.clone());

    child_models.iter().for_each(|(property, model_type)| {
        let (gen_name, iterations) = if let DT::List(nested) = model_type {
            match nested.borrow() {
                DT::Model(next_model_name) => (next_model_name.clone(), 5),
                _ => return
            }
        } else if let DT::Model(next_model_name) = model_type {
            (next_model_name.clone(), 1)
        } else {
            return;
        };

        for _ in 0..iterations {
            let mut next_model_ctx = GenContext {
                parent_context: Some(Box::new(ctx.clone())),
                parent_model: Some(GenData {
                    model: model.clone(),
                    data: model_data.clone(),
                }),
                models: HashMap::new(),
            };
            generate_model_data(gen_name.clone(), &spec.get_definition(&gen_name), &mut next_model_ctx, &spec);
            ctx.merge_model_data(&mut next_model_ctx.models);
        }

    });
}


pub fn write_output(folder: &PathBuf, data: ModelDataMap, spec: Specification, out_type: OutputType) {
    create_dir_all(&folder);
    match out_type {
        OutputType::CSV => {
            data.iter().for_each(|(type_name, model_list)| {
                let mut path = PathBuf::from(&folder);
                path.push(&type_name);
                path = path.with_extension(out_type.as_extension());

                let mut file = File::create(path).unwrap();
                let ordering = spec.get_serialize_ref(&type_name);
                let mut writer = Csv::from_writer(file);
                for data_set in model_list {
                    let mut row: Vec<String> = Vec::new();
                    if let Some(order) = ordering {
                        for key in order.iter() {
                            row.push(data_set.get(key).unwrap_or(&String::from("null")).to_string());
                        }
                    } else {
                        data_set.values().for_each(|v| row.push(v.clone()));
                    }
                    writer.write_record(&row).unwrap();
                }
            })
        },
        OutputType::JSON => unimplemented!("JSON has not been implemented just yet"),
    }
}
