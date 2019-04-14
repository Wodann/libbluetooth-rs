#![cfg(unix)]
#![allow(bad_style)]

#[macro_use]
extern crate nix;

extern crate libc;

#[macro_use]
mod macros;

pub mod bluetooth;
pub mod bnep;
pub mod cmtp;
pub mod hci;
pub mod hci_lib;
pub mod hidp;
pub mod l2cap;
pub mod rfcomm;
pub mod sco;
pub mod sdp;
pub mod sdp_lib;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
