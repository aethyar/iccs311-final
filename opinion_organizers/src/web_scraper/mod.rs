//! This module contains the functions for web scraping in parallel
//! of any input Vector of String.
//! 
//! The code uses a parallel iterator over the input to perform WebScraping quickly.

extern crate reqwest;
extern crate scraper;
extern crate rayon;

use scraper::{Html, Selector};
use rayon::prelude::*;

// Web_scraping
// we'll be scraping from the individual reviews on - https://www.themoviedb.org/ only.

/// Input is a link that contains other reviews
/// through the link get the other reviews by navigating (in parallel) to each of the reviews website by clicking on "read the rest"
/// ensure each review is not added to the slice of String more than once
/// store the review websites in a slice
#[allow(dead_code)]
pub fn review_collection(masterurl: &str) -> Vec<String> {
    let req = reqwest::blocking::get(masterurl)
        .unwrap_or_else(|err| panic!("the URL does not exist: {}", err)); // error message for when we cannot establish a connection
    
    let doc_body = Html::parse_document(&req.text().unwrap());

    let select_underline = Selector::parse(".underline").unwrap();

    let mut reviews = Vec::new();

    for element_underline in doc_body.select(&select_underline) { // go through all the underline classes to get the links
        if let Some(href) = element_underline.value().attr("href") { // href = hypertext reference thats the url we must follow. it is an attribute.
            if href.starts_with("/review/") {
                let complete_url = format!("https://www.themoviedb.org{}", href);
                reviews.push(complete_url);
            }
        }
    }
    web_to_string(reviews)
}

/// Checks if input url's have a successful connection
/// scrapes all paragraphs in the body of the individual movie review (this program is fine tuned to this specific movie review website)
/// then merges it into 1 string and repeats in parallel the same thing with the other movie reviews before mapping it to a Vector containing all of the movie reviews
///
/// Returns a Vector of Strings containing the reviews
#[allow(dead_code)]
pub fn web_to_string(urls: Vec<String>) -> Vec<String> {    // we take in a Vec of links that can be variale in amount

    urls.par_iter().map(|url| {
    
        let req = reqwest::blocking::get(url.as_str())
            .unwrap_or_else(|_err| panic!("the URL does not exist")); // error message for when we cannot establish a connection
        // if it's a success you will not see the error message

        let doc_body = Html::parse_document(&req.text().unwrap());  // parsing the document itself

        let review_selector = Selector::parse("div.content p").unwrap();    // finding the specific content we want to scrape using either a class or html ID

        let mut review_texts = Vec::new(); // this empty vector is created so it can later hold all of the texts for output

        for review_selector in doc_body.select(&review_selector){
            let review_text = review_selector.text().collect::<Vec<_>>();
            for i in 0..review_text.len() { // iterator
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
        let urls = vec!["https://www.themoviedb.org/review/58a231c5925141179e000674".to_string(), 
                                    "https://www.themoviedb.org/review/5d340e7a2f8d090388d21ff2".to_string(),
        ];
        let result = web_to_string(urls);
        assert_eq!(result.len(), 2);
        assert!(result[0].contains("The Imperial March"));
        assert!(result[1].contains("Back in 1977"));
    }

    #[test]
    #[should_panic]
    fn invalid_url_test() {
        let urls_with_error_handling = vec!["https://oehiuhfiuehfiucnwiunweonc.com".to_string(),];
        let reviews_with_error_handling = web_to_string(urls_with_error_handling);
        assert_eq!(reviews_with_error_handling[0], "Well, it actually has a title, what the Darth Vader theme. And that title is \n\"The Imperial March\", composed by the great John Williams, whom, \nas many of you may already know, also composed the theme music for \n\"Jaws\" - that legendary score simply titled, \"Main Title (Theme \nFrom Jaws)\".");
    }

}
