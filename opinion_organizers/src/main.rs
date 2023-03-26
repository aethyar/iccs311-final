use crate::sentiment_analysis::get_sentiment_counts;
use crate::web_scraper::web_to_string;

mod web_scraper;
mod sentiment_analysis;

fn main() {
    // ask for user input for a link to process

    let urls = &["https://www.themoviedb.org/review/58a231c5925141179e000674", "https://www.themoviedb.org/review/5d340e7a2f8d090388d21ff2"];
    let vec_texts = web_to_string(urls);
    println!("{:?}", vec_texts[1]);
    let test_string = "This is a sad test.";  // replace with web_scraper output

    let (pos, neg): (usize, usize) = get_sentiment_counts(test_string.to_owned());  // replace with parallel code if web_scraper output is a collection of Strings

    // can make the output fancier later
    if pos == 0 { print!("{}", neg) }
    else if neg == 0 { print!("{}", pos) }
    else { print!("{}", pos/neg) }
}
