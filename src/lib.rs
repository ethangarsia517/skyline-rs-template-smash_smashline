#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    unused_macros
)]

use smashline::*;

mod kirby;
mod palutena;
pub mod vars;

#[skyline::main(name = "smashline_test")]
pub fn main() {
    kirby::install();
    palutena::install();

}