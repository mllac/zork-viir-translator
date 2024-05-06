use std::str::FromStr;

use crate::naive_grammer::Prefix;

impl FromStr for Prefix {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.to_lowercase().ends_with("rg") {
            Ok(Self::Rg)
        } else if s.ends_with("rb") {
            Ok(Self::Rb)
        } else {
            Err(())
        }
    }
}

impl TryInto<String> for Prefix {
    type Error = ();

    fn try_into(self) -> Result<String, Self::Error> {
        match self {
            Self::Rg => Ok("rg".into()),
            Self::Rb => Ok("rb".into()),
        }
    }
}
