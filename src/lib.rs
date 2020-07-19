use anyhow::Result;
pub use blackjack_proc_macro::DataFrame;

#[macro_export]
macro_rules! blackjack_init {
    () => {
        #[derive(Default, Debug)]
        pub struct DataFrame<T> {
            pub values: Vec<T>,
        }
    };
}

pub trait InnerJoin {
    type Left;
    type Right;

    /// Implement joining two rows from different dataframes
    /// into another row type
    fn join(left: &Self::Left, right: &Self::Right) -> Result<Self>
    where
        Self: Sized;
}

#[macro_export]
macro_rules! join {
    ($left:ident -><- $right:ident) => {
        <DataFrame<_>>::from_iter(
            $left
                .values
                .iter()
                .map(|left_row| {
                    $right
                        .values
                        .iter()
                        .filter_map(move |right_row| InnerJoin::join(left_row, right_row).ok())
                })
                .flatten(),
        )
    };
}
