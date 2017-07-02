extern crate portmidi as pm;
extern crate ref_slice;

mod color;
mod font;
mod launchpad;

pub use launchpad::*;
pub use color::*;
pub use font::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
