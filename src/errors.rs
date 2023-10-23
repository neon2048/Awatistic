use std::fmt;

#[derive(Debug, Clone)]
pub enum AwawaLoadError {
    AwawaParseError,
    MissingInitialAwaError,
    MalformedAwatismError,
    UnknownAwatismError(u8),
}

#[derive(Debug, Clone)]
pub enum AwawaError {
    BubbleAbyssEmpty,
    BubbleAbyssOutOfBounds,
    InvalidAwasciiCodeError(i32),
    InvalidAwasciiCharError(char),
    ReadLineError,
    NotANumberError(String),
    UnknownAwatismError(String),
    MissingArgumentError,
    InvalidArgumentError,
    InvalidLabelError(u8),
    EndOfProgramError(),
}

pub type AwawaResult = Result<(), AwawaError>;
pub type AwawaLoadResult = Result<(), AwawaLoadError>;

impl fmt::Display for AwawaLoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AwawaParseError => {
                write!(f, "Only 'awa' and 'wa' are allowed")
            }
            Self::MissingInitialAwaError => write!(f, "Missing initial 'awa'"),
            Self::MalformedAwatismError => write!(f, "Awatism malformed: arguments missing"),
            Self::UnknownAwatismError(awa) => write!(f, "Awatism {awa} not implemented"),
        }
    }
}
impl fmt::Display for AwawaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BubbleAbyssEmpty => write!(f, "Bubble Abyss accessed but it is empty"),
            Self::BubbleAbyssOutOfBounds => write!(f, "Bubble Abyss accessed out of bounds"),
            Self::InvalidAwasciiCodeError(val) => write!(f, "Invalid AwaSCII code {val}"),
            Self::InvalidAwasciiCharError(val) => {
                write!(f, "Character {val} cannot be represented in AwaSCII")
            }
            Self::ReadLineError => write!(f, "Failed to read input"),
            Self::NotANumberError(s) => write!(f, "Text '{s}' cannot be converted to number"),
            Self::UnknownAwatismError(s) => write!(f, "Unknown awatism '{s}'"),
            Self::MissingArgumentError => write!(f, "Awatism requires one or more arguments"),
            Self::InvalidArgumentError => write!(f, "The argument is invalid"),

            Self::InvalidLabelError(l) => write!(f, "Label {l} is invalid"),
            Self::EndOfProgramError() => write!(f, "Program ended"),
        }
    }
}
