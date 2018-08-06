use std::fmt;
use std::str::FromStr;

#[derive(GraphQLEnum, Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize, DieselTypes, EnumIterator)]
#[graphql(name = "Status", description = "Current moderation status")]
pub enum ModerationStatus {
    Draft,
    Moderation,
    Decline,
    Published,
}

impl FromStr for ModerationStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "draft" => Ok(ModerationStatus::Draft),
            "moderation" => Ok(ModerationStatus::Moderation),
            "decline" => Ok(ModerationStatus::Decline),
            "published" => Ok(ModerationStatus::Published),
            _ => Err(()),
        }
    }
}

impl fmt::Display for ModerationStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ModerationStatus::Draft => write!(f, "draft"),
            ModerationStatus::Moderation => write!(f, "moderation"),
            ModerationStatus::Decline => write!(f, "decline"),
            ModerationStatus::Published => write!(f, "published"),
        }
    }
}

impl ModerationStatus {
    pub fn as_vec() -> Vec<ModerationStatus> {
        vec![
            ModerationStatus::Draft,
            ModerationStatus::Moderation,
            ModerationStatus::Decline,
            ModerationStatus::Published,
        ]
    }
}
