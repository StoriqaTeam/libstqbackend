use std::fmt;
use std::str::FromStr;

use juniper::FieldError;

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
