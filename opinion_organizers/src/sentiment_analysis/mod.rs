use std::sync::atomic::{AtomicUsize, Ordering};
use rayon::{str::ParallelString, prelude::ParallelIterator};

pub mod dataset;
use dataset::{POSITIVE_WORDS, NEGATIVE_WORDS};

#[allow(dead_code)]
fn is_negative_word(word: &str) -> bool {
    false
}

#[allow(dead_code)]
fn is_positive_word(word: &str) -> bool {
    false
}

#[allow(dead_code)]
pub fn get_sentiment_counts(input: String) -> (usize, usize) {
    let positive_count = AtomicUsize::new(0);
    let negative_count = AtomicUsize::new(0);

    input.par_split_whitespace().for_each(|word| {
        if is_positive_word(word) { positive_count.fetch_add(1, Ordering::SeqCst); }
        else if is_negative_word(word) { negative_count.fetch_add(1, Ordering::SeqCst); }
    });

    (positive_count.into_inner(), negative_count.into_inner())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        assert_eq!((3, 0), get_sentiment_counts("Wow, a good word: Nice.".to_owned()));
    }
}
