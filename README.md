# black-jack

## BlackJack is under development and not meant to be used; aside from those curious & daring few. :)

[![crates.io](http://meritbadge.herokuapp.com/black-jack)](https://crates.io/crates/black-jack) 
[![Build Status](https://travis-ci.org/milesgranger/black-jack.svg?branch=master)](https://travis-ci.org/milesgranger/black-jack) 
[![Coverage Status](https://coveralls.io/repos/github/milesgranger/black-jack/badge.svg?branch=master)](https://coveralls.io/github/milesgranger/black-jack?branch=master)
![License](https://img.shields.io/badge/license-Unlicense-green.svg) 



[Rust API Documentation](https://docs.rs/black-jack)

---

The idea is to implement a light-weight alternative to Python's [Pandas](https://pandas.pydata.org/), in and for the Rust community. Pandas is a fantastic library, but has some memory issues as well has having a _massive_ API; and attempting to make something comparable in Rust, just _feels_ right. :)

...however, for the most part this will serve as an area to learn more Rust
and connecting it with Python via the [Lumber-Jack](https://github.com/milesgranger/lumber-jack), which is basically a wrapper to Black-Jack. This crate was initially born there, but better design insisted it be moved to its own crate/repository. ;)

---

## Development

- Rust >= 1.29, although older versions are expected to work as well.
- [GSL](https://www.gnu.org/software/gsl/) ~= 2.4
    - Fedora: `sudo dnf install gsl-devel`
    - Ubuntu: `sudo apt-get install libgsl-dev`
    - [Windows Install Instructions](https://www.gnu.org/software/gsl/extras/native_win_builds.html)

---

## Contributing

All contributions are welcome. Patrons of this project are expect to treat all
others with respect and dignity; acknowledging there will be differences of opinion
and strive to provide a welcoming environment for others, regardless of skill level.

Additionally, all contributions will be given under the [Unlicense](http://unlicense.org/) agreement.