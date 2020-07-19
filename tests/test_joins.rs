use anyhow::{bail, Result};
use std::iter::FromIterator;

use blackjack::{blackjack_init, join, DataFrame, InnerJoin};

blackjack_init!();

#[test]
fn test_basic_inner() {
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

    let left: DataFrame<Temp> = DataFrame::from_iter(vec![1, 2, 4].into_iter().map(|day| Temp {
        day,
        temp: (day * 4) as f32,
    }));

    let right: DataFrame<Rain> = DataFrame::from_iter(vec![1, 2, 3].into_iter().map(|day| Rain {
        day,
        rain: (day * 2) as f32,
    }));

    #[derive(DataFrame, Default)]
    struct TempAndRain {
        day: u8,
        temp: f32,
        rain: f32,
    }

    impl InnerJoin for TempAndRain {
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

    // Inner join, days should be 1, 2
    let joined: DataFrame<TempAndRain> = join!(left -><- right);
    assert_eq!(joined.len(), 2);
    assert_eq!(joined.day().collect::<Vec<&u8>>(), vec![&1, &2]);

    // Right join, days should be 1, 2, 4

    // Left join, days should be 1, 2, 3

    // Outer join, days should be 1, 2, 3, 4
}
