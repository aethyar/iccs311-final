use std::sync::atomic::{AtomicUsize, Ordering};
use rayon::{str::ParallelString, prelude::ParallelIterator};
use regex::Regex;

mod dataset;

fn is_negative_word(word: &str) -> bool {
    dataset::NEGATIVE_WORDS.contains(word)
}

fn is_positive_word(word: &str) -> bool {
    dataset::POSITIVE_WORDS.contains(word)
}

pub fn get_sentiment_counts(input: String) -> (usize, usize) {
    let positive_count = AtomicUsize::new(0);
    let negative_count = AtomicUsize::new(0);

    let re = Regex::new(r"[^\w*-]").unwrap();

    input.par_split_whitespace().for_each(|word| {
        let clean_word = re.replace_all(word, "").to_lowercase();
        print!("{}\n", clean_word);
        if is_positive_word(&clean_word) { positive_count.fetch_add(1, Ordering::SeqCst); }
        else if is_negative_word(&clean_word) { negative_count.fetch_add(1, Ordering::SeqCst); }
    });

    (positive_count.into_inner(), negative_count.into_inner())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        assert_eq!((3, 0), get_sentiment_counts("Wow, a good word: Nice.".to_owned()));
        assert_eq!((0, 2), get_sentiment_counts("He's a sad drop-out.".to_owned()));
        assert_eq!((0, 2), get_sentiment_counts("bull****, bull----".to_owned()));
    }
}
