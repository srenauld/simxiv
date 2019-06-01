#![feature(proc_macro_hygiene)]
#![recursion_limit = "128"]
#[macro_use] extern crate simxiv_spelldata;
extern crate simxiv_prelude;

use simxiv_prelude::{Action, ActionTarget, Moment};
use std::collections::HashMap;
#[test]
fn it_works() {
    let data = embed_spelldata!("spell_data/tests/action.csv");
    assert_eq!(2 + 2, 4);
}