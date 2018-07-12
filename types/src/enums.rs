#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize, DieselTypes)]
pub enum Role {
    Superuser,
    User,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize, DieselTypes)]
pub enum MerchantType {
    Store,
    User,
}
