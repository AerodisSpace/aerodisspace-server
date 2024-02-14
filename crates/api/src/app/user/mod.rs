pub mod auth;
pub mod get_user;

pub mod user_roles {
    use std::str::FromStr;

    use serde::{Deserialize, Serialize};
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub enum UserRoles {
        Admin,
        Developer,
        FreeUser,
        PremiumUser,
    }
    impl ToString for UserRoles {
        fn to_string(&self) -> String {
            match self {
                UserRoles::Admin => "Admin".to_string(),
                UserRoles::Developer => "Developer".to_string(),
                UserRoles::FreeUser => "FreeUser".to_string(),
                UserRoles::PremiumUser => "PremiumUser".to_string(),
            }
        }
    }
    impl FromStr for UserRoles {
        type Err = ();
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "Admin" => Ok(UserRoles::Admin),
                "Developer" => Ok(UserRoles::Developer),
                "FreeUser" => Ok(UserRoles::FreeUser),
                "PremiumUser" => Ok(UserRoles::PremiumUser),
                _ => Err(()),
            }
        }
    }
}
