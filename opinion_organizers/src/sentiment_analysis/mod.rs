use rayon::{prelude::ParallelIterator, str::ParallelString};
use regex::Regex;
use std::sync::atomic::{AtomicUsize, Ordering};

mod dataset;

/// Checks if input word is part of `POSITIVE_WORDS` set
///
/// Returns `true` if satisfied, `false` otherwise
fn is_positive_word(word: &str) -> bool {
    dataset::POSITIVE_WORDS.contains(word)
}

/// Checks if input word is part of `NEGATIVE_WORDS` set
///
/// Returns `true` if satisfied, `false` otherwise
fn is_negative_word(word: &str) -> bool {
    dataset::NEGATIVE_WORDS.contains(word)
}

/// Produces the sentiment counts for the input string by checking all the words from the input
/// in parallel
///
/// Returns the total count for positive and negative words
pub fn get_sentiment_counts(input: String) -> (usize, usize) {
    let positive_count = AtomicUsize::new(0);
    let negative_count = AtomicUsize::new(0);

    let re = Regex::new(r"[^\w*-]").unwrap();

    input.par_split_whitespace().for_each(|word| {
        let clean_word = re.replace_all(word, "").to_lowercase();
        if is_positive_word(&clean_word) {
            positive_count.fetch_add(1, Ordering::SeqCst);
        } else if is_negative_word(&clean_word) {
            negative_count.fetch_add(1, Ordering::SeqCst);
        }
    });

    (positive_count.into_inner(), negative_count.into_inner())
}

/// Sample tests below
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        assert_eq!(
            (3, 0),
            get_sentiment_counts("Wow, a good word: Nice.".to_owned())
        );
        assert_eq!(
            (0, 2),
            get_sentiment_counts("He's a sad drop-out.".to_owned())
        );
        assert_eq!(
            (0, 2),
            get_sentiment_counts("bull****, bull----".to_owned())
        );
    }

    #[test]
    fn extra_test() {
        assert_eq!((1, 0), get_sentiment_counts(
            "It's a contemplative adventure and an emotional exploration that captivated me from its opening moments..."
            .to_owned()
        ));
        assert_eq!((2, 0), get_sentiment_counts(
            "While not all-together perfect, the film represents a monumental cinematic achievement that deserves to be placed high within the caliber of Nolan's filmography."
            .to_owned()
        ));
        assert_eq!(
            (1, 1),
            get_sentiment_counts("As spectacular as it is flawed.".to_owned())
        );
    }

    #[test]
    fn essay_test() {
        assert_eq!((24, 16), get_sentiment_counts(
            "I enjoyed watching Quantumania. It's a mostly solid and fairly entertaining movie. But relative to MCU standards, it's a bit underwhelming and feels like more of a 
            throwaway movie than something memorable I will keep coming back to.

            My biggest issue is that I wasn't fully engaged for at least the first half of the movie. I was enjoying it but I wasn't locked in. The story was unfocussed. When the 
            villain becomes prevalent, the movie jumps a level. But it doesn't completely make up for the much weaker first half.
            
            The other thing that surprised me was not being mesmerized by the visuals. It's very creative and I didn't notice poor CGI like some others. But it didn't wow me like it 
            frequently does. Maybe we're just getting spoiled with the visuals in movies like Doctor Strange 2 and more recently Avatar 2.
            
            There is still a lot of good here. It's pretty funny but doesn't overdo it. This is the second movie in a row where the MCU seems to have dialed back on the 'insert joke here' 
            strategy. It's probably a result of the negative reaction from the cartoonish Thor 4. Also, I love the villain. I won't say who in case you're smart (and crazy) like me and 
            don't watch trailers. But he's so damn awesome.
            
            As someone who loves almost everything in the MCU, Quantumania is near the bottom of my MCU rankings. And it's making me a little nervous that it comes only two movies after 
            Thor 4 which is below it (5 stars). But I still have faith and am not joining the MCU doom and gloom crowd. Phase 4 has three of my favorite entries in the entire 
            MCU (Shang-Chi, Loki S01, No Way Home) and I only disliked two (Eternals, Thor 4). They just need to be more consistent while also not playing it so safe."
            .to_owned()
        ));
    }
}
