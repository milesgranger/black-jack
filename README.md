# black-jack

#### BlackJack is under development and not meant to be used; aside from those curious & daring few. :)

[![crates.io](http://meritbadge.herokuapp.com/black-jack)](https://crates.io/crates/black-jack) 
[![Build Status](https://travis-ci.com/milesgranger/black-jack.svg?branch=master)](https://travis-ci.com/milesgranger/black-jack) 
[![Coverage Status](https://coveralls.io/repos/github/milesgranger/black-jack/badge.svg?branch=master)](https://coveralls.io/github/milesgranger/black-jack?branch=master)
[![Dependabot Status](https://api.dependabot.com/badges/status?host=github&repo=milesgranger/black-jack)](https://dependabot.com)
[![License](https://img.shields.io/badge/license-Unlicense-green.svg)](http://unlicense.org/)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](https://en.wikipedia.org/wiki/MIT_License)  



[Rust API Documentation](https://docs.rs/black-jack)

---

BlackJack strives to be a full featured crate for general data processing.


_Long term_ goal is to create a lightweight [Pandas](https://pandas.pydata.org/) equivalent
by and for the Rust community, but with slight differences in focus...


The project strives for a few key principles. When any implementation decisions are to be made,
they are made with these principles in mind, and in this order:
1. **Memory efficiency**
    - Minimize memory use at every opportunity.
2. **Usability**
    - Strive for ergonomics; often done by modeling the `Pandas` API where possible.
3. **Speedy**
    - It comes naturally most times with Rust. :)


Eventually we'll have a Python wrapper: [Lumber-Jack](https://github.com/milesgranger/lumber-jack)
associated with this crate, but that time will come.

### Example use:

<!--
```rust,skeptic-template
extern crate blackjack;

use blackjack::prelude::*;

fn main() {{
    {}
}}
```
-->

```rust

// We have a dataframe, of course...
let mut df = DataFrame::new();

// Make some series, of different types
let series_i32: Series<i32> = Series::arange(0, 5);
let mut series_f64: Series<f64> = Series::from_vec(vec![1.0, 2.0, 3.0, 4.0, 5.0]);

// You can set a series name!
series_f64.set_name("my-series");

// Or not... 
assert_eq!(series_i32.name(), None);

// And add them to the dataframe
df.add_column(series_f64).unwrap();
df.add_column(series_i32).unwrap();

// And then get a reference to a Series
let series_f64_ref: &Series<f64> = df.get_column("my-series").unwrap();

// and a lot more...

```
---

## Development

- Rust >= 1.29, although older versions are expected to work as well.
- [GSL](https://www.gnu.org/software/gsl/) ~= 2.4
    - Fedora: `sudo dnf install gsl-devel`
    - Ubuntu: `sudo apt-get install libgsl-dev`
    - [Windows Install Instructions](https://www.gnu.org/software/gsl/extras/native_win_builds.html)

---

## Contributing

All contributions are welcome. Contributors of this project are expected to treat all
others with respect and dignity; acknowledging there will be differences of opinion
and strive to provide a welcoming environment for others, regardless of skill level.

Additionally, all contributions, unless otherwise stated, will be given under the [Unlicense](http://unlicense.org/) 
and/or [MIT](https://en.wikipedia.org/wiki/MIT_License) licenses.
