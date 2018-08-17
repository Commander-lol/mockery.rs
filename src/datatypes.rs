use uuid;
use std;

/// An Enum that represents all of the possible random data generation types.
/// Where the type is a struct, it should be represented as a nested map type, where the outer
/// map contains only a property correlating to the struct name, and it's value should be a nested
/// map with the struct values.
///
/// # Examples
///
/// Representing a person with a randomly generated full name, as well as an age between 18 and 25
/// (e.g. seeding a database for under-25 travelcards) would be written like this in JSON:
///
/// ```json
/// {
///     "person": {
///         "name": "FullName",
///         "age": {
///             "NumberBetween": {
///                 "min": 18,
///                 "max": 25
///             }
///         }
///     }
/// }
/// ```
///
/// Similarly, if you were defining your models in YAML, and wanted to define a company contact card,
/// you would write something like:
///
/// ```yaml
/// company:
///     name: Company
///     contact_email: Email
///     contact_number: PhoneNumber
///     address: StreetAddress
///     postcode: Postcode
///     cover_image:
///         LoremPicsum:
///             width: 500
///             height: 200
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
    /// # Examples
    ///
    /// `{ "digits": 3 }` will generate a number between 100 and 999, inclusive
    Number {
        /// The number of digits the generated number should have, to get a number with an appropriate
        /// order of magnitude
        digits: usize,
    },
    /// Generate a random number between the minimum and maximum boundaries
    NumberBetween {
        /// The minimum boundary for the generated number. This boundary is inclusive
        min: usize,
        /// The maximum boundary for the generated number. This boundary is exclusive
        max: usize,
    },
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
    /// Geenrates a valid postcode
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
        RandomData::LatLong => format!(r#"[{}, {}]"#, fake!(Address.latitude), fake!(Address.longitude)),
        RandomData::LongLat => format!(r#"[{}, {}]"#, fake!(Address.longitude), fake!(Address.latitude)),
        RandomData::GeoPoint => format!(r#"POINT({} {})"#, fake!(Address.longitude), fake!(Address.latitude)),
        RandomData::Postcode => format!("{}", fake!(Address.postcode)),
        RandomData::FullAddress => format!("{}, {}, {}", fake!(Address.street_address), fake!(Address.city), fake!(Address.postcode)),
        RandomData::UUID4 => format!("{}", uuid::Uuid::new_v4()),
        RandomData::PhoneNumber => format!("{}", fake!(PhoneNumber.phone_number)),
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
