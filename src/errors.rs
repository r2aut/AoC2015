// тип данных для ошибок
// #[derive(PartialEq)]
pub enum AoCError {
    // желаемые опции ошибок
    FileIOEror(std::io::Error),
    ParseIntError(std::num::ParseIntError),
    RegexError(regex::Error),
    RecvError(std::sync::mpsc::RecvError),
    IntCodeError(String),
    OtherError(String), // чтобы покрыть возможно неучтеные или не интересные ошибки
}

// реализация трейта std::fmt::Display
impl std::fmt::Display for AoCError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FileIOEror(err) => write!(f, "{}", err),
            Self::ParseIntError(err) => write!(f, "{}", err),
            Self::RegexError(err) => write!(f, "{}", err),
            Self::RecvError(err) => write!(f, "{}", err),
            Self::IntCodeError(mess) => write!(f, "{}", mess),
            Self::OtherError(mess) => write!(f, "{}", mess),
        }
    }
}

// реализация трейта std::fmt::Debug
impl std::fmt::Debug for AoCError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FileIOEror(err) => write!(f, "{}", err),
            Self::ParseIntError(err) => write!(f, "{}", err),
            Self::RegexError(err) => write!(f, "{}", err),
            Self::RecvError(err) => write!(f, "{}", err),
            Self::IntCodeError(mess) => write!(f, "{}", mess),
            Self::OtherError(mess) => write!(f, "{}", mess),
        }
    }
}

// реализация трейта std::error::Error
impl std::error::Error for AoCError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FileIOEror(_mess) => None,
            Self::ParseIntError(err) => Some(err),
            Self::RegexError(err) => Some(err),
            Self::RecvError(err) => Some(err),
            Self::IntCodeError(_) => None,
            Self::OtherError(_) => None,
        }
    }
}

// реальзация конвертации из других ошибок
impl From<std::io::Error> for AoCError {
    fn from(err: std::io::Error) -> Self {
        Self::FileIOEror(err)
    }
}

impl From<std::num::ParseIntError> for AoCError {
    fn from(err: std::num::ParseIntError) -> Self {
        Self::ParseIntError(err)
    }
}

impl From<regex::Error> for AoCError {
    fn from(err: regex::Error) -> Self {
        Self::RegexError(err)
    }
}
impl From<std::sync::mpsc::RecvError> for AoCError {
    fn from(err: std::sync::mpsc::RecvError) -> Self {
        Self::RecvError(err)
    }
}
