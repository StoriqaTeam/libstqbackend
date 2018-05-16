extern crate juniper;
extern crate std;

use std::fmt;
use std::str::FromStr;

use juniper::FieldError;

pub enum Model {
    User,
    JWT,
    Store,
    Product,
    BaseProduct,
    UserRoles,
    Attribute,
    Category,
    CartProduct,
    CartStore,
    SearchCategory,
    WizardStore,
    ModeratorProductComment,
    ModeratorStoreComment,
    UserDeliveryAddress,
}

impl fmt::Display for Model {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Model::User => write!(f, "user"),
            Model::JWT => write!(f, "jwt"),
            Model::Store => write!(f, "store"),
            Model::Product => write!(f, "product"),
            Model::BaseProduct => write!(f, "base_product"),
            Model::UserRoles => write!(f, "user_roles"),
            Model::Attribute => write!(f, "attribute"),
            Model::Category => write!(f, "category"),
            Model::CartProduct => write!(f, "cart_product"),
            Model::CartStore => write!(f, "cart_store"),
            Model::SearchCategory => write!(f, "search_category"),
            Model::WizardStore => write!(f, "wizard_store"),
            Model::ModeratorProductComment => write!(f, "moderator_product_comment"),
            Model::ModeratorStoreComment => write!(f, "moderator_store_comment"),
            Model::UserDeliveryAddress => write!(f, "user_delivery_address"),
        }
    }
}

impl FromStr for Model {
    type Err = FieldError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "user" => Ok(Model::User),
            "jwt" => Ok(Model::JWT),
            "store" => Ok(Model::Store),
            "product" => Ok(Model::Product),
            "base_product" => Ok(Model::BaseProduct),
            "user_roles" => Ok(Model::UserRoles),
            "attribute" => Ok(Model::Attribute),
            "category" => Ok(Model::Category),
            "cart_product" => Ok(Model::CartProduct),
            "cart_store" => Ok(Model::CartStore),
            "search_category" => Ok(Model::SearchCategory),
            "wizard_store" => Ok(Model::WizardStore),
            "moderator_product_comment" => Ok(Model::ModeratorProductComment),
            "moderator_store_comment" => Ok(Model::ModeratorStoreComment),
            "user_delivery_address" => Ok(Model::UserDeliveryAddress),
            _ => Err(FieldError::new(
                "Unknown model",
                graphql_value!({ "code": 300, "details": {
                        format!("Can not resolve model name. Unknown model: '{}'", s)
                        }}),
            )),
        }
    }
}

impl Model {
    pub fn to_url(&self) -> String {
        match *self {
            Model::User => "users".to_string(),
            Model::JWT => "jwt".to_string(),
            Model::Store => "stores".to_string(),
            Model::Product => "products".to_string(),
            Model::BaseProduct => "base_products".to_string(),
            Model::UserRoles => "user_roles".to_string(),
            Model::Attribute => "attributes".to_string(),
            Model::Category => "categories".to_string(),
            Model::CartProduct => "cart_products".to_string(),
            Model::CartStore => "cart_store".to_string(),
            Model::SearchCategory => "search_category".to_string(),
            Model::WizardStore => "wizard_stores".to_string(),
            Model::ModeratorProductComment => "moderator_product_comments".to_string(),
            Model::ModeratorStoreComment => "moderator_store_comments".to_string(),
            Model::UserDeliveryAddress => "user_delivery_address".to_string(),
        }
    }
}
