use std::fmt::{Debug, Display, Formatter};
use crate::parser::Rule;

impl Display for Rule {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}
