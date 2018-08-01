use uuid;
use chrono::prelude::*;
use std::{self, fmt};
use serde::{self, Deserialize, Deserializer};
use serde::de::{self, Visitor};
use serde_json;
use regex::Regex;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum RandomData {
    FirstName,
    LastName,
    FullName,
    Email,
    Number { digits: usize },
    NumberBetween { min: usize, max: usize },
    Company,
    City,
    StreetAddress,
    Latitude,
    Longitude,
    LatLong,
    Postcode,
    UUID4,
    PhoneNumber,
    LoremPicsum { width: Option<usize>, height: Option<usize>, grayscale: Option<bool> },
    DateBetween { format: Option<String> }
}
impl RandomData {
    pub fn into_data(self) -> String {
        generate_fake_data(self)
    }
}
impl std::string::ToString for RandomData {
    fn to_string(&self) -> String {
        self.clone().into_data()
    }
}

lazy_static! {
    static ref PARENT_TYPE_CAPTURE: Regex = Regex::new(r#"ParentType\((\w+)\)"#).unwrap();
    static ref REFERENCE_CAPTURE: Regex = Regex::new(r#"Reference\((\w+)\)"#).unwrap();
}

#[derive(Clone, Debug, Serialize)]
pub enum Constraint {
    Parent,
    ParentType(String),
    Reference(String),
}
struct ConstraintVisitor;
#[derive(Clone, Debug)]
struct CustomError;
impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.write_str("Oh no")
    }
}
impl de::Error for CustomError {
    fn custom<T>(msg: T) -> Self
        where
            T: fmt::Display {
        CustomError
    }
}
impl std::error::Error for CustomError {}

impl <'de>Visitor <'de>for ConstraintVisitor {
    type Value = Constraint;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Parent, ParentType(model_name) or Reference(field_ref)")
    }

    fn visit_str<E>(self, v: &str) -> Result<<Self as Visitor>::Value, E> where
        E: de::Error, {
//        let v =
        if v == "Parent" {
            Ok(Constraint::Parent)
        } else if let Some(parent_type) = PARENT_TYPE_CAPTURE.captures(v) {
            parent_type.get(1);
            Err(CustomError {} as de::Error)
        } else {
            Err(CustomError {} as de::Error)
        }
    }
}

impl <'de> Deserialize<'de> for Constraint {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error> where
        D: Deserializer<'de> {
        unimplemented!()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum DataType {
    Data(RandomData),
    Constraint(Constraint, String),
    Model(String),
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DataField {
    data: DataType,
    reference: Option<String>,
}
impl std::str::FromStr for DataField {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, <Self as std::str::FromStr>::Err> {
        serde_json::from_str(s).unwrap()
    }
}

pub fn generate_fake_data(spec: RandomData) -> String {
    match spec {
        RandomData::FirstName => format!("{}", fake!(Name.first_name)),
        RandomData::LastName => format!("{}", fake!(Name.last_name)),
        RandomData::FullName => format!("{}", fake!(Name.name)),
        RandomData::Email => format!("{}", fake!(Internet.safe_email)),
        RandomData::Number { digits } => format!("{}", fake!(Number.number(digits))),
        RandomData::NumberBetween { min, max } => format!("{}", fake!(Number.between(min, max))),
        RandomData::Company => format!("{}", fake!(Company.name)),
        RandomData::City => format!("{}", fake!(Address.city)),
        RandomData::StreetAddress => format!("{}", fake!(Address.street_address)),
        RandomData::Latitude => format!("{}", fake!(Address.latitude)),
        RandomData::Longitude => format!("{}", fake!(Address.longitude)),
        RandomData::LatLong => format!(r#"{{ "lat": "{}", "long": "{}" }}"#, fake!(Address.latitude), fake!(Address.longitude)),
        RandomData::Postcode => format!("{}", fake!(Address.postcode)),
        RandomData::UUID4 => format!("{}", uuid::Uuid::new_v4()),
        RandomData::PhoneNumber => format!("{}", fake!(PhoneNumber.phone_number)),
        RandomData::LoremPicsum { width, height, grayscale } => format!(
            "https://picusm.photos/{}{}/{}",
            if grayscale.unwrap_or(false) { "g/" } else { "" },
            width.unwrap_or(200),
            height.unwrap_or(200)
        ),
//        DataType::DateBetween{ format } => {
//            let now = Utc::now();
//            format!("{}", fake!(Chrono.between(format, &now.to_rfc3339(), &now.to_rfc3339())))
//        },
        _ => String::new(),
    }
}