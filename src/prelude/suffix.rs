use std::str::FromStr;

use crate::naive_grammer::Suffix;

impl FromStr for Suffix {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.to_lowercase().ends_with("omp") {
            Ok(Self::Omp)
        } else if s.to_lowercase().ends_with("iop") {
            Ok(Self::Iop)
        } else {
            Err(())
        }
    }
}
