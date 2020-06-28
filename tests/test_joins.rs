use anyhow::{bail, Result};
use std::iter::FromIterator;

use blackjack::{blackjack_init, join, DataFrame, Join};

blackjack_init!();

#[test]
fn test_basic_left() {
    #[derive(DataFrame, Default, Clone)]
    struct Rain {
        day: u8,
        rain: f32,
    }

    #[derive(DataFrame, Default, Clone)]
    struct Temp {
        day: u8,
        temp: f32,
    }

    let rain: DataFrame<Rain> = DataFrame::from_iter((0..3).map(|day| Rain {
        day,
        rain: (day * 2) as f32,
    }));
    let temp: DataFrame<Temp> = DataFrame::from_iter((0..3).map(|day| Temp {
        day,
        temp: (day * 4) as f32,
    }));

    #[derive(DataFrame, Default)]
    struct TempAndRain {
        day: u8,
        temp: f32,
        rain: f32,
    }

    impl Join for TempAndRain {
        type Left = Temp;
        type Right = Rain;

        fn join(left: &Self::Left, right: &Self::Right) -> Result<Self>
        where
            Self: Sized,
        {
            if left.day != right.day {
                bail!("Keys do not match")
            } else {
                Ok(Self {
                    day: left.day,
                    temp: left.temp,
                    rain: right.rain,
                })
            }
        }
    }

    let joined: DataFrame<TempAndRain> = join!(temp <- rain);
    assert_eq!(joined.len(), 3);
}
