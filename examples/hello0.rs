//! Example based on hello0 from http://wiki.call-cc.org/iup-tutor

extern crate iup;

use iup::prelude::*;
use iup::control::Label;

fn main() {
    iup::with_iup(|| Dialog::new(Label::with_title("Hello, world!")).show()).unwrap()
}
