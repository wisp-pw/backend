use crate::prelude::*;

#[macro_export]
macro_rules! handle_err {
    ($x:expr) => {
        |e| {
            error!("{:?}", e);
            $x
        }
    };
}

pub use handle_err;
