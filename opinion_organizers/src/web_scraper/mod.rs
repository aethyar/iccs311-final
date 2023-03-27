//! This module contains the functions for web scraping in parallel
//! of any input Vector of String.
//! 
//! The code uses a parallel iterator over the input to perform WebScraping quickly.

extern crate reqwest;
extern crate scraper;
extern crate rayon;

use scraper::{Html, Selector};
use rayon::prelude::*;

// use scraper::Selector;
// use scraper::Html;

// Web_scraping
// we'll be scraping from the individual reviews on - https://www.themoviedb.org/ only.

/// Checks if input url's have a successful connection
/// scrapes all paragraphs in the body of the individual movie review (this program is fine tuned to this specific movie review website)
/// then merges it into 1 string and repeats in parallel the same thing with the other movie reviews before mapping it to a Vector containing all of the movie reviews
///
/// Returns a Vector of Strings containing the reviews
#[allow(dead_code)]
pub fn web_to_string(urls: &[&str]) -> Vec<String> {    // we take in a Vec of links that can be variale in amount

    urls.par_iter().map(|url| {
    
        let req = reqwest::blocking::get(*url)
            .unwrap_or_else(|err| panic!("Couldn't load the url: {}", err)); // error message for when we cannot establish a connection
        // if it's a success you will not see the error message

        let doc_body = Html::parse_document(&req.text().unwrap());  // parsing the document itself

        let review_selector = Selector::parse("div.content p").unwrap();    // finding the specific content we want to scrape using either a class or html ID

        let mut review_texts = Vec::new(); // this empty vector is created so it can later hold all of the texts for output

        for review_selector in doc_body.select(&review_selector){
            let review_text = review_selector.text().collect::<Vec<_>>();
            for i in 0..review_text.len() { // iterator
                //println!("{}", review_text[i]);
                review_texts.push(review_text[i].to_string()); // pushing them into one
            }
        }
        review_texts.join("") // joining all of them in 1 to return as a String
    })
    .collect() // puts it into the vector
}

/// Sample tests below
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_web_to_string() {
        let urls = &["https://www.themoviedb.org/review/58a231c5925141179e000674", "https://www.themoviedb.org/review/5d340e7a2f8d090388d21ff2"];
        let result = web_to_string(urls);
        assert_eq!(result.len(), 2);
        assert!(result[0].contains("The Imperial March"));
        assert!(result[1].contains("Great film, great soundtrack"));
    }

    #[test]
    fn invalid_url_test() {
        let urls_with_error_handling = vec!["https://www.themoviedb.org/review/58a231c5925141179e000674", "https://oehiuhfiuehfiucnwiunweonc.com"];
        let reviews_with_error_handling = web_to_string(&urls_with_error_handling);
        assert_eq!(reviews_with_error_handling[0], "Well, it actually has a title, what the Darth Vader theme. And that title is \n\"The Imperial March\", composed by the great John Williams, whom, \nas many of you may already know, also composed the theme music for \n\"Jaws\" - that legendary score simply titled, \"Main Title (Theme \nFrom Jaws)\".");
        assert!(reviews_with_error_handling[1].is_empty());
    }

}
