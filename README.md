# black-jack

##### While PRs are welcome, the approach taken only allows for concrete types (String, f64, i64, ...) I'm not sure this is the way to go. I want to think that using anything which implements `serde::{Serialize, Deserialize}` and better use of traits may have more flexibility, just not sure how to do that yet. The project is not abandoned, just in limbo. :-)

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

```rust,skt-default

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

```

## Read a CSV file:
Also supports reading `.gz` files

```rust,skt-default

// Define the path to file
let path: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/medium_csv.csv");

// Use the `Reader` to read the dataframe
let df = Reader::new(&path).read().expect("Failed to read file");

// Get a refrence to a specific column and assert the sum of that series
let series2: &Series<i32> = df.get_column("col2").unwrap();

assert_eq!(series2.sum(), 3000);

```

## Query/filter a dataframe

```rust,skt-default
let mut s1 = Series::from(0..5);
s1.set_name("col1");

let mut s2 = Series::from(10..15);
s2.set_name("col2");

let mut s3 = Series::from_vec(vec![
    "foo".to_string(),
    "bar".to_string(),
    "foo".to_string(),
    "bar".to_string(),
    "foo".to_string(),
]);
s3.set_name("col3");

let mut df = DataFrame::new();
assert!(df.add_column(s1).is_ok());
assert!(df.add_column(s2).is_ok());
assert!(df.add_column(s3).is_ok());

// Before filtering, we're len 5 and first element of 'col1' is 0
assert_eq!(df.len(), 5);

df.filter_by_row(|row| row["col1"] == Datum::I32(&0));

// After filtering, we're len 4 and first element of 'col1' is now 1
assert_eq!(df.len(), 4);

// Filter by string foo,
df.filter_by_row(|row| row["col3"] != Datum::STR(&"foo".to_string()));
assert_eq!(df.len(), 2);
```


## and a whole lot more..


---

## Development

- Rust >= 1.31
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
