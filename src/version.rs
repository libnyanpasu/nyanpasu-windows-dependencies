use std::{fmt, str::FromStr};

use snafu::Snafu;

/// A four-part version, as Windows packages and runtime installers report it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Version {
    pub major: u16,
    pub minor: u16,
    pub build: u16,
    pub revision: u16,
}

impl Version {
    pub const fn new(major: u16, minor: u16, build: u16, revision: u16) -> Self {
        Self {
            major,
            minor,
            build,
            revision,
        }
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}.{}.{}.{}",
            self.major, self.minor, self.build, self.revision
        )
    }
}

/// The text is not four dot-separated numbers that each fit in 16 bits.
#[derive(Debug, Snafu)]
#[snafu(display("{text:?} is not a four-part version"))]
pub struct ParseVersionError {
    text: String,
}

impl FromStr for Version {
    type Err = ParseVersionError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut parts = text.split('.').map(str::parse::<u16>);
        match (
            parts.next(),
            parts.next(),
            parts.next(),
            parts.next(),
            parts.next(),
        ) {
            (Some(Ok(major)), Some(Ok(minor)), Some(Ok(build)), Some(Ok(revision)), None) => {
                Ok(Self::new(major, minor, build, revision))
            }
            _ => ParseVersionSnafu { text }.fail(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compares_component_by_component() {
        assert!(Version::new(2, 5, 1, 0) > Version::new(2, 4, 9, 9));
        assert!(Version::new(14, 51, 36247, 0) > Version::new(14, 40, 33728, 0));
    }

    #[test]
    fn parses_and_displays_four_parts() {
        let version: Version = "155.0.4283.62".parse().unwrap();
        assert_eq!(version, Version::new(155, 0, 4283, 62));
        assert_eq!(version.to_string(), "155.0.4283.62");
    }

    #[test]
    fn rejects_other_shapes() {
        for text in ["", "1.2.3", "1.2.3.4.5", "v14.51.36247.00", "1.2.3.65536"] {
            assert!(text.parse::<Version>().is_err(), "{text}");
        }
    }
}
