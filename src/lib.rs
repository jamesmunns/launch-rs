mod color;

pub use color::*;

mod error;
#[doc(inline)]
pub use error::{Error, Result};

#[cfg(feature = "mk2")]
pub mod mk2;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
