humanity [![Build Status](https://travis-ci.org/RoxasShadow/humanity.svg?branch=master)](https://travis-ci.org/RoxasShadow/humanity) [![crates.io](http://meritbadge.herokuapp.com/humanity)](https://crates.io/crates/humanity)
========

Generate [humans.txt](http://humanstxt.org) from given repository's contributors list.

# Usage
To compile *humanity*, [Rust Nightly](https://www.rust-lang.org/downloads.html) is required.

```sh
$ cargo build --release
$ ./target/release/humanity iron/params
```

If the program crashes, it's because GitHub APIs allow you to execute a very low amount of requests.
What you have to do is just generating a new [token](https://github.com/settings/tokens/new) (pick
a name, leave everything unchecked and press "Generate token") and run `export GITHUB_ACCESS_TOKEN=`,
appending the received token.

If you need access to your private repositories, check the "repo" flag for your token.

# Customizations
It's possible to create additional fetchers implenting the `HumanityBearer` trait and also redefining
the output format using a custom implementation of `ToString` for `Humanity` and `User`.
