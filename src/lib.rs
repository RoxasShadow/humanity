#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate hyper;
extern crate serde;
extern crate serde_json;

mod humanity;
mod github;

pub use humanity::{Humanity, HumanityBearer, User};
pub use github::{GitHub, GitHubOwner, GitHubContributor};
