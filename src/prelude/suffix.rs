use std::str::FromStr;

use crate::naive_grammer::Suffix;

impl FromStr for Suffix {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.to_lowercase().ends_with("omp") {
            Ok(Self::Omp)
        } else if s.to_lowercase().ends_with("having") {
            Ok(Self::Iop)
        } else {
            Err(())
        }
    }
}

impl TryInto<String> for Suffix {
    type Error = ();

    fn try_into(self) -> Result<String, Self::Error> {
        match self {
            Self::Omp => Ok("omp".into()),
            Self::Iop => Ok("iop".into()),
        }
    }
}
