use std::{error::Error, fmt::Display, io, string::FromUtf8Error};

#[derive(Debug)]
pub enum UuidError {
    OsRngUnavailable(io::Error),
    FromUtf8Error(FromUtf8Error),
    ClockBeforeUnixEpoch,
}

impl Error for UuidError {}

impl Display for UuidError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UuidError::OsRngUnavailable(error) => {
                write!(f, "OSの乱数生成器が利用できません: {error}")
            }
            UuidError::FromUtf8Error(error) => write!(f, "UTF8への変換に失敗しました: {error}"),
            UuidError::ClockBeforeUnixEpoch => write!(
                f,
                "OSの時刻設定が1970-01-01 00:00:00 UTCより前になっています"
            ),
        }
    }
}

impl From<io::Error> for UuidError {
    fn from(value: io::Error) -> Self {
        Self::OsRngUnavailable(value)
    }
}

impl From<FromUtf8Error> for UuidError {
    fn from(value: FromUtf8Error) -> Self {
        Self::FromUtf8Error(value)
    }
}
