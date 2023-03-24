use std::sync::atomic::AtomicUsize;

pub mod dataset;

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

    (positive_count.into_inner(), negative_count.into_inner())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        assert_eq!((3, 0), get_sentiment_counts("Wow, a good and happy word.".to_owned()));
    }
}
