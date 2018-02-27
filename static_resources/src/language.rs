use std::fmt;
use std::str::FromStr;

use juniper::FieldError;

#[derive(Copy, Clone)]
pub enum Language {
    English = 1,
    Chinese,
    German,
    Russian,
    Spanish,
    French,
    Korean,
    Portuguese,
    Japanese,
}

#[derive(GraphQLObject, Serialize, Deserialize, Debug)]
pub struct LanguageGraphQl {
    pub key: i32,
    pub name: String,
}

impl LanguageGraphQl {
    pub fn new(key: i32, name: String) -> Self {
        Self { key, name }
    }
}

impl Language {
    pub fn as_vec() -> Vec<LanguageGraphQl> {
        vec![
            Language::English,
            Language::Chinese,
            Language::German,
            Language::Russian,
            Language::Spanish,
            Language::French,
            Language::Korean,
            Language::Portuguese,
            Language::Japanese,
        ].into_iter()
            .map(|value| LanguageGraphQl::new(value as i32, value.to_string()))
            .collect()
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lang = match *self {
            Language::English => "english",
            Language::Chinese => "chinese",
            Language::German => "german",
            Language::Russian => "russian",
            Language::Spanish => "spanish",
            Language::French => "french",
            Language::Korean => "korean",
            Language::Portuguese => "portuguese",
            Language::Japanese => "japanese",
        };
        write!(f, "{}", lang)
    }
}

impl FromStr for Language {
    type Err = FieldError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "english" => Language::English,
            "chinese" => Language::Chinese,
            "german" => Language::German,
            "russian" => Language::Russian,
            "spanish" => Language::Spanish,
            "french" => Language::French,
            "korean" => Language::Korean,
            "portuguese" => Language::Portuguese,
            "japanese" => Language::Japanese,
            _ => {
                return Err(FieldError::new(
                    "Unknown service",
                    graphql_value!({ "code": 300, "details": {
                        format!("Can not resolve service name. Unknown service: '{}'", s)
                        }}),
                ))
            }
        })
    }
}
