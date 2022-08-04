use crate::prelude::*;

pub trait ResultExtensions<T, E>
where
    E: std::fmt::Display,
{
    ///
    /// map and log the original error
    ///
    fn handle_err<V>(self, to: V) -> Result<T, V>;
}

impl<T, E> ResultExtensions<T, E> for Result<T, E>
where
    E: std::fmt::Display,
{
    fn handle_err<V>(self, to: V) -> Result<T, V> {
        match self {
            Ok(t) => Ok(t),
            Err(e) => {
                error!("{e}");
                Err(to)
            }
        }
    }
}
