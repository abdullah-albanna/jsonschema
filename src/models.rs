use std::collections::HashMap;

use create_json_schema_struct_macro::create_json_schema_struct;
use serde::{de::Visitor, Deserialize, Deserializer, Serialize, Serializer};

/// used to identify what type is current schema
///
/// ```rust
///
/// jsonschema!{
///     type: object,
///     ...
/// }
/// ```
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum JsonSchemaTypes {
    Object,
    String,
    Array,
    Number,

    // we make it the default so to know if it's fresh with ::default or has already been set
    #[default]
    None,
}

create_json_schema_struct! {
    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct JsonSchema {
        #[serde(rename = "type")]
        pub ty: JsonSchemaTypes,

        pub title: Option<String>,
        pub description: Option<String>,

        #[serde(rename = "default")]
        pub default: Option<JsonSchemaValues>,

        #[serde(rename = "examples")]
        pub examples: Option<Vec<String>>,

        #[serde(rename = "enum")]
        pub enum_values: Option<Vec<JsonSchemaValues>>,

        #[serde(rename = "const")]
        pub const_value: Option<JsonSchemaValues>,

        pub properties: Option<HashMap<String, JsonSchema>>,
        pub required: Option<Vec<String>>,
        pub min_lenght: Option<usize>,
        pub max_lenght: Option<usize>,
        pub pattern: Option<String>,
        pub format: Option<Formats>,
        pub minimum: Option<usize>,
        pub maximum: Option<usize>,
        pub items: Option<Box<JsonSchema>>,
        pub min_items: Option<usize>,
        pub max_items: Option<usize>,
        pub unique_items: Option<bool>,
        pub contains: Option<Box<JsonSchema>>,

    }
}

/// the main struct holding all the data about every root and nested schemas
// #[derive(Clone, Debug, Default, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct JsonSchema {
//     #[serde(rename = "type")]
//     pub ty: JsonSchemaTypes,

//     #[serde(skip_serializing_if = "::std::option::Option::is_none")]
//     pub title: Option<String>,

//     #[serde(skip_serializing_if = "::std::option::Option::is_none")]
//     pub description: Option<String>,

//     #[serde(skip_serializing_if = "::std::option::Option::is_none")]
//     pub default: Option<JsonSchemaValues>,

//     #[serde(skip_serializing_if = "::std::option::Option::is_none")]
//     pub examples: Option<Vec<String>>,

//     #[serde(skip_serializing_if = "::std::option::Option::is_none")]
//     #[serde(rename = "enum")]
//     pub enum_values: Option<Vec<JsonSchemaValues>>,

//     #[serde(skip_serializing_if = "::std::option::Option::is_none")]
//     #[serde(rename = "const")]
//     pub const_value: Option<JsonSchemaValues>,

//     // object specific keys
//     #[serde(skip_serializing_if = "::std::option::Option::is_none")]
//     pub properties: Option<HashMap<String, JsonSchema>>,

//     #[serde(skip_serializing_if = "::std::option::Option::is_none")]
//     pub required: Option<Vec<String>>,

//     // string specific keys
//     #[serde(skip_serializing_if = "::std::option::Option::is_none")]
//     pub min_lenght: Option<usize>,

//     #[serde(skip_serializing_if = "::std::option::Option::is_none")]
//     pub max_lenght: Option<usize>,

//     #[serde(skip_serializing_if = "::std::option::Option::is_none")]
//     pub pattern: Option<String>,

//     #[serde(skip_serializing_if = "::std::option::Option::is_none")]
//     pub format: Option<Formats>,

//     // number specific keys
//     #[serde(skip_serializing_if = "::std::option::Option::is_none")]
//     pub minimum: Option<usize>,

//     #[serde(skip_serializing_if = "::std::option::Option::is_none")]
//     pub maximum: Option<usize>,

//     // array specific keys
//     #[serde(skip_serializing_if = "::std::option::Option::is_none")]
//     pub items: Option<Box<JsonSchema>>,

//     #[serde(skip_serializing_if = "::std::option::Option::is_none")]
//     pub min_items: Option<usize>,

//     #[serde(skip_serializing_if = "::std::option::Option::is_none")]
//     pub max_items: Option<usize>,

//     #[serde(skip_serializing_if = "::std::option::Option::is_none")]
//     pub unique_items: Option<bool>,

//     #[serde(skip_serializing_if = "::std::option::Option::is_none")]
//     pub contains: Option<Box<JsonSchema>>,

//     // span tracking fields
//     // it's a key value spans
//     #[serde(skip)]
//     pub current_key_span: Option<proc_macro2::Span>,

//     /// those are used to report better errors, to exactly tell where the errors are
//     #[serde(skip)]
//     pub ty_span: Option<(proc_macro2::Span, proc_macro2::Span)>,
//     #[serde(skip)]
//     pub title_span: Option<(proc_macro2::Span, proc_macro2::Span)>,
//     #[serde(skip)]
//     pub required_span: Option<(proc_macro2::Span, proc_macro2::Span)>,
//     #[serde(skip)]
//     pub description_span: Option<(proc_macro2::Span, proc_macro2::Span)>,
//     #[serde(skip)]
//     pub items_span: Option<(proc_macro2::Span, proc_macro2::Span)>,
//     #[serde(skip)]
//     pub properties_span: Option<(proc_macro2::Span, proc_macro2::Span)>,
//     #[serde(skip)]
//     pub default_span: Option<(proc_macro2::Span, proc_macro2::Span)>,
//     #[serde(skip)]
//     pub examples_span: Option<(proc_macro2::Span, proc_macro2::Span)>,
//     #[serde(skip)]
//     pub enum_values_span: Option<(proc_macro2::Span, proc_macro2::Span)>,
//     #[serde(skip)]
//     pub const_value_span: Option<(proc_macro2::Span, proc_macro2::Span)>,
//     #[serde(skip)]
//     pub min_lenght_span: Option<(proc_macro2::Span, proc_macro2::Span)>,
//     #[serde(skip)]
//     pub max_lenght_span: Option<(proc_macro2::Span, proc_macro2::Span)>,
//     #[serde(skip)]
//     pub pattern_span: Option<(proc_macro2::Span, proc_macro2::Span)>,
//     #[serde(skip)]
//     pub format_span: Option<(proc_macro2::Span, proc_macro2::Span)>,
//     #[serde(skip)]
//     pub minimum_span: Option<(proc_macro2::Span, proc_macro2::Span)>,
//     #[serde(skip)]
//     pub maximum_span: Option<(proc_macro2::Span, proc_macro2::Span)>,
//     #[serde(skip)]
//     pub min_items_span: Option<(proc_macro2::Span, proc_macro2::Span)>,
//     #[serde(skip)]
//     pub max_items_span: Option<(proc_macro2::Span, proc_macro2::Span)>,
//     #[serde(skip)]
//     pub unique_items_span: Option<(proc_macro2::Span, proc_macro2::Span)>,
//     #[serde(skip)]
//     pub contains_span: Option<(proc_macro2::Span, proc_macro2::Span)>,

//     // indicate the depth of the schema starting from 1 as root
//     #[serde(skip)]
//     pub depth: usize,
// }

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Formats {
    Date,
    Time,
    DateTime,
    Email,
    Hostname,
    Ipv4,
    Ipv6,
    Uri,
}

impl std::fmt::Display for Formats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Formats::Date => f.write_str("date"),
            Formats::Time => f.write_str("time"),
            Formats::DateTime => f.write_str("date-time"),
            Formats::Email => f.write_str("email"),
            Formats::Hostname => f.write_str("hostname"),
            Formats::Ipv4 => f.write_str("ipv4"),
            Formats::Ipv6 => f.write_str("ipv6"),
            Formats::Uri => f.write_str("uri"),
        }
    }
}

/// contains every ident that's considered as a keyword
///
/// ```rust
/// jsonschema!{
///     type: ...,
///     title: "...",
///     default: "...",
///     required: [...],
///     description: "...",
///     items: ..., // you can also use { type: ... }
///     properties: {
///         "...": {...}
///     }
///    
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub enum JsonSchemaKeywords {
    Type,
    Title,
    Required,
    Description,
    Items,
    Properties,
    Default,
    Examples,
    Enum,
    Const,
    MinLength,
    MaxLenght,
    Pattern,
    Format,
    Minimum,
    Maximum,
    MinItems,
    MaxItems,
    UniqueItems,
    Contains,
    Struct,
}

/// stores what's after the `:`
///
/// ```rust
/// jsonschema!{
///     type: ... // the `...` is the value
/// }
///     
///    
/// ```
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JsonSchemaValues {
    #[serde(
        serialize_with = "serialize_ident",
        deserialize_with = "deserialize_ident"
    )]
    Ident(syn::Ident),
    Str(String),
    Number(i64),
    Bool(bool),
    Char(char),
    Array(Vec<JsonSchemaValues>),
}

impl JsonSchemaValues {
    pub fn get_str(&self) -> Option<&String> {
        match self {
            JsonSchemaValues::Str(ref s) => Some(s),
            _ => None,
        }
    }
}

#[allow(dead_code)]
impl JsonSchemaTypes {
    pub fn is_none(&self) -> bool {
        matches!(self, JsonSchemaTypes::None)
    }
}

impl std::fmt::Display for JsonSchemaTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsonSchemaTypes::Array => f.write_str("array"),
            JsonSchemaTypes::Object => f.write_str("object"),
            JsonSchemaTypes::String => f.write_str("string"),
            JsonSchemaTypes::Number => f.write_str("number"),
            JsonSchemaTypes::None => f.write_str("null"),
        }
    }
}

impl std::fmt::Display for JsonSchemaValues {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsonSchemaValues::Ident(ident) => f.write_str(&ident.to_string()),
            JsonSchemaValues::Str(s) => f.write_str(&s),
            JsonSchemaValues::Number(num) => f.write_str(&format!("{}", num)),
            JsonSchemaValues::Bool(b) => f.write_str(&format!("{}", b)),
            JsonSchemaValues::Char(c) => f.write_str(&format!("{}", c)),
            JsonSchemaValues::Array(array) => f.write_str(&format!("{:?}", array)),
        }
    }
}

// Custom serializer for Ident
fn serialize_ident<S>(ident: &syn::Ident, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&ident.to_string())
}

// Custom deserializer for Ident
fn deserialize_ident<'de, D>(deserializer: D) -> Result<syn::Ident, D::Error>
where
    D: Deserializer<'de>,
{
    struct IdentVisitor;

    impl<'de> Visitor<'de> for IdentVisitor {
        type Value = syn::Ident;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(formatter, "a string representing an identifier")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(syn::Ident::new(value, proc_macro2::Span::call_site()))
        }
    }

    deserializer.deserialize_str(IdentVisitor)
}
