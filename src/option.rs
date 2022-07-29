use color_eyre::Result;

pub trait OptionExtension<T> {
    ///
    /// Returns an error if the ```Option``` has ```Some```
    /// 
    fn ok_err<E>(self, err: E) -> Result<(), E>;
}

impl<T> OptionExtension<T> for Option<T> {
    fn ok_err<E>(self, err: E) -> Result<(), E> {
        match self {
            Some(_) => Err(err),
            None => Ok(()),
        }
    }
}