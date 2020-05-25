pub use blackjack_proc_macro::DataFrame;

#[macro_export]
macro_rules! blackjack_init {
    [] => {
        #[derive(Default, Debug)]
        pub struct DataFrame<T> {
            values: Vec<T>
        }
    }
}
