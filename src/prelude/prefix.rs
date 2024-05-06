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

