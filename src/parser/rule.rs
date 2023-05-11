use std::fmt::{Debug, Display, Formatter};
use super::Rule;

impl Display for Rule {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}
