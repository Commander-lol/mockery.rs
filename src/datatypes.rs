use uuid;
use serde_derive::{Deserialize, Serialize};
use fake::{Faker, Fake};
use fake::faker;
use rand::Rng;
use std::ops::Add;

/// An Enum that represents all of the possible random data generation types.
/// Where the type is a struct it should be written in a specification as a nested map type,
/// where the outer map contains a single property correlating to the struct variant
/// with value should be a map containing the struct values.
///
/// # Examples
///
/// ## Basic Model (JSON)
///
/// To model a company identification card that contains basic employee information using simple
/// types:
///
/// ```json
/// {
///     "information_card": {
///         "name": "FullName",
///         "contact_email": "Email",
///         "contact_number": "PhoneNumber",
///         "address": "FullAddress"
///     }
/// }
/// ```
///
/// ## Model With Struct Types (JSON)
///
/// To use complex data types that take arguments, you need to write the property values as
/// nested map types. This example models an Under-25 Travelcard for a train company, where
/// the age of the person needs to be between 18 and 25. The `NumberBetween` type takes two
/// arguments:
///
/// ```json
/// {
///     "travelcard": {
///         "name": "FullName",
///         "address": "FullAddress",
///         "age": {
///             "NumberBetween": {
///                 "min": 18,
///                 "max": 25
///             }
///         }
///     }
/// }
/// ```
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum RandomData {
    /// Generates a latin first name
    FirstName,
    /// Generates a latin surname
    LastName,
    /// Generates a combination of a `FirstName` and `LastName`, combined with a space between them
    FullName,
    /// Generates a safe email address
    Email,
    /// Generates an integer with the given number of digits.
    ///
    /// ## Examples
    ///
    /// `Number: { "digits": 3 }` will generate a number between 100 and 999, inclusive
    Number {
        /// The number of digits the generated number should have, to get a number with an appropriate
        /// order of magnitude
        digits: usize,
    },
    /// Generate a random number between the minimum and maximum boundaries
    ///
    /// ## Examples
    ///
    /// `NumberBetween: { "min": 23, "max": 50 }` will generate a number between 23 and 49, inclusive
    NumberBetween {
        /// The minimum boundary for the generated number. This boundary is inclusive
        min: usize,
        /// The maximum boundary for the generated number. This boundary is exclusive
        max: usize,
    },
    /// Generate a number of paragraphs containing raw lorem ipsum text (No formatting)
    Paragraphs {
        /// The number of paragraphs to generate. Defaults to 1
        amount: Option<usize>,
    },
    /// Generate a single paragraph of lorem ipsum text. This is a convenience for specifying
    /// `Paragraphs` as a simple type without options
    Paragraph,
    /// Generate a number of sentences containing raw lorem ipsum text (No formatting)
    Sentences {
        /// The number of sentences to generate. Defaults to 1
        amount: Option<usize>,
    },
    /// Generate a single sentence of lorem ipsum text. This is a convenience for specifying
    /// `Sentences` as a simple type without options
    Sentence,
    /// Generates the name of a local company - usually an amalgamation of name-parts
    Company,
    /// Generates the name of a city
    City,
    /// Generates a sensible street address (including house number and street name components)
    StreetAddress,
    /// Generates a valid latitude component
    Latitude,
    /// Generates a valid longitude component
    Longitude,
    /// Generates a coordinate pair, formatted as a JSON array of (Latitude, Longitude)
    LatLong,
    /// Generates a coordinate pair, formatted as a JSON array of (Longitude, Latitude)
    LongLat,
    /// Generates a GeoJson point object using the WKT formatting for postgis recognition and insertion
    GeoPoint,
    /// Generates a valid postcode
    Postcode,
    /// Generates an address with `StreetAddress`, `City` and `PostCode` components, separated by commas
    FullAddress,
    /// Generates a valid V4 UUID, generally useful for object IDs
    UUID4,
    /// Generates a valid phone number
    PhoneNumber,
    /// Generates a URL to a random picture with the given dimensions and optional greyscale mode.
    /// Where one of the size values is absent, the image will be a square as dictated by the
    /// other value that is present. Where both are absent, the image will be a 200x200 pixel square.
    LoremPicsum { width: Option<usize>, height: Option<usize>, grayscale: Option<bool> },
    NullValue,
    String { content: String },
    Reference {
        model: String,
        field: String,
    },
}

impl RandomData {
    /// Consumes the `RandomData` instance and turns it into a random piece of data, corresponding to
    /// its type
    pub fn into_data(self) -> String {
        generate_fake_data(self)
    }
}

impl std::string::ToString for RandomData {
    fn to_string(&self) -> String {
        self.clone().into_data()
    }
}

/// Create a number of a certain length. The value is returned as a string for display,
/// and can therefore represent a number of arbitrary length at the expense of higher memory
/// consumption.
///
/// The first digit will be in the range 1-9, whilst the following digits will be between 0-9.
///
/// # Examples
///
/// ```rust
/// let three_digit_number = number_with_length(3);
/// println!("{}", three_digit_number);
/// ```
fn number_with_length(length: usize) -> String {
    let mut random = rand::thread_rng();
    let mut buffer = String::with_capacity(length);
    buffer = buffer + &format!("{}", random.gen_range(1, 10));

    for _ in 0..length - 1 {
        buffer = buffer + &format!("{}", random.gen_range(0, 10));
    }

    buffer
}

#[test]
fn generate_number_format_of_correct_length() {
    assert_eq!(number_with_length(1).len(), 1);
    assert_eq!(number_with_length(2).len(), 2);
    assert_eq!(number_with_length(10).len(), 10);
    assert_eq!(number_with_length(1000).len(), 1000);
}

/// Use a `RandomData` definition to generate a random string of data
///
/// # Examples
///
/// ```rust
/// use mockery::datatypes::{RandomData, generate_fake_data};
/// println!(
///     "Hello {}, your new email address is {}",
///     generate_fake_data(RandomData::FullName),
///     generate_fake_data(RandomData::Email)
/// )
/// ```
pub fn generate_fake_data(spec: RandomData) -> String {
    match spec {
        RandomData::FirstName => format!("{}", faker::name::en::FirstName().fake::<String>()),
        RandomData::LastName => format!("{}", faker::name::en::LastName().fake::<String>()),
        RandomData::FullName => format!("{}", faker::name::en::Name().fake::<String>()),
        RandomData::Email => format!("{}", faker::internet::en::SafeEmail().fake::<String>()),
        RandomData::Number { digits } => format!("{}", number_with_length(digits)),
        RandomData::NumberBetween { min, max } => format!("{}", rand::thread_rng().gen_range(min, max)),
        RandomData::Paragraph => faker::lorem::en::Paragraph(1..2).fake::<String>(),
        RandomData::Paragraphs { amount } => {
            let val = amount.unwrap_or(1usize);
            format!("{}", faker::lorem::en::Paragraph(val..val+1).fake::<String>())
        },
        RandomData::Sentence => faker::lorem::en::Sentence(1..2).fake::<String>(),
        RandomData::Sentences { amount } => {
            let val = amount.unwrap_or(1usize);
            format!("{}", faker::lorem::en::Sentence(val..val+1).fake::<String>())
        },
        RandomData::Company => format!("{}", faker::company::en::CompanyName().fake::<String>()),
        RandomData::City => format!("{}", faker::address::en::CityName().fake::<String>()),
        RandomData::StreetAddress => format!("{}", faker::address::en::StreetName().fake::<String>()),
        RandomData::Latitude => format!("{}", faker::address::en::Latitude().fake::<String>()),
        RandomData::Longitude => format!("{}", faker::address::en::Longitude().fake::<String>()),
        RandomData::LatLong => format!(r#"[{}, {}]"#, faker::address::en::Latitude().fake::<String>(), faker::address::en::Longitude().fake::<String>()),
        RandomData::LongLat => format!(r#"[{}, {}]"#, faker::address::en::Longitude().fake::<String>(), faker::address::en::Latitude().fake::<String>()),
        RandomData::GeoPoint => format!(r#"POINT({} {})"#, faker::address::en::Longitude().fake::<String>(), faker::address::en::Latitude().fake::<String>()),
        RandomData::Postcode => format!("{}", faker::address::en::PostCode().fake::<String>()),
        RandomData::FullAddress => format!(
            "{}, {}, {}",
            faker::address::en::StreetName().fake::<String>(),
            faker::address::en::CityName().fake::<String>(),
            faker::address::en::PostCode().fake::<String>()
        ),
        RandomData::UUID4 => format!("{}", uuid::Uuid::new_v4()),
        RandomData::PhoneNumber => format!("{}", faker::phone_number::en::PhoneNumber().fake::<String>()),
        RandomData::LoremPicsum { width, height, grayscale } => format!(
            "https://picusm.photos/{}{}/{}",
            if grayscale.unwrap_or(false) { "g/" } else { "" },
            width.unwrap_or(200),
            height.unwrap_or(200)
        ),
        RandomData::NullValue => format!("null"),
        RandomData::String { content } => content.clone(),
        RandomData::Reference { .. } => format!("null"),
    }
}
