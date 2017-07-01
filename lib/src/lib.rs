extern crate portmidi as pm;
extern crate ref_slice;

mod color;
mod launchpad;

pub use launchpad::*;
pub use color::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
