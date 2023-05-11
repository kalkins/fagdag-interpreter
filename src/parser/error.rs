use std::fmt::{Display, Formatter};
use std::panic::Location;
use itertools::Itertools;
use pest::iterators::Pair;
use pest::RuleType;

pub struct LineInfo {
    position: (usize, usize),
    line: String,
}

impl<R: RuleType> From<&Pair<'_, R>> for LineInfo {
    fn from(value: &Pair<R>) -> Self {
        LineInfo {
            position: value.line_col(),
            line: value.as_str().to_string()
        }
    }
}

impl Display for LineInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error at ({}, {}): {}", self.position.0, self.position.1, self.line)
    }
}

pub struct CallerInfo {
    file: String,
    line: String,
    column: u32,
}

impl From<&Location<'_>> for CallerInfo {
    fn from(value: &Location) -> Self {
        CallerInfo {
            file: value.file().to_string(),
            line: value.line().to_string(),
            column: value.column(),
        }
    }
}

impl Display for CallerInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.file, self.line, self.column)
    }
}

pub enum ParseError {
    Error {
        message: String,
        line: LineInfo,
    },
    UnexpectedEnd {
        line: LineInfo,
        caller: CallerInfo,
    },
    UnexpectedRule {
        line: LineInfo,
        caller: CallerInfo,
        rule: String,
    },
    ErrorList(Vec<ParseError>),
}

impl ParseError {
    pub fn from_pair<R: RuleType, M: ToString>(pair: &Pair<R>, message: M) -> ParseError {
        Self::Error {
            message: message.to_string(),
            line: pair.into(),
        }
    }

    #[track_caller]
    pub fn wrong_rule<R: RuleType + Display>(pair: &Pair<R>, rule: R) -> ParseError {
        Self::UnexpectedRule {
            line: pair.into(),
            caller: Location::caller().into(),
            rule: rule.to_string(),
        }
    }

    #[track_caller]
    pub fn end<R: RuleType>(pair: &Pair<R>) -> ParseError {
        Self::UnexpectedEnd {
            line: pair.into(),
            caller: Location::caller().into(),
        }
    }

    pub fn merge<T, I: IntoIterator<Item=Result<T, Self>>>(iter: I) -> Result<Vec<T>, Self> {
        let (successes, errors): (Vec<T>, Vec<Self>) = iter.into_iter().partition_result();

        if errors.is_empty() {
            Ok(successes)
        } else {
            Err(Self::from_iter(errors))
        }
    }
}

impl FromIterator<ParseError> for ParseError {
    fn from_iter<T: IntoIterator<Item=ParseError>>(iter: T) -> Self {
        ParseError::ErrorList(iter.into_iter().collect())
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Error {message, line} => {
                write!(f, "{line}\n\t{message}")
            }
            Self::UnexpectedEnd {line, caller} => {
                write!(f, "{line}: Unexpected end of inner tokens. Probably a grammar error. Called from {caller}")
            }
            Self::UnexpectedRule {line, caller, rule} => {
                write!(f, "{line}: Unexpected rule {rule}. Called from {caller}")
            }
            Self::ErrorList(list) => {
                list.iter().try_for_each(|e| e.fmt(f))
            }
        }
    }
}
