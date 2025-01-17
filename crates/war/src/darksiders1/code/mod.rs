#![allow(clippy::module_inception, clippy::module_name_repetitions)]

macro_rules! prop {
    ($name:literal, $value:expr $(,)?) => {
        ($name.to_string(), $value)
    };
}

pub mod library;
pub mod vigil;
