use prelude::*;

#[test]
fn test_alterations_one_hot() {
    /*
        Test the alterations split_n_hot_encode function
    */
    let words = vec!["hi, hello".to_string(), "hi, bye".to_string()];
    let (unique_words, one_hot) = alterations::split_n_hot_encode(
        words.clone(), ",".to_string(), 0
    );

    // Test that the first vector of unique words found contains "hi"
    println!("Got unique words: {:?}", unique_words);
    assert_eq!(unique_words.contains(&"hi".to_string()), true);

    // Test that the first one-hot array sums to 2, for "hi" and "hello"
    println!("Got one-hot: {:?}", one_hot);
    assert_eq!(one_hot[0].iter().fold(0, |sum, &val| sum + val), 2);
}