#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate hyper;
extern crate serde;
extern crate serde_json;

pub mod humanity;
pub mod github;
