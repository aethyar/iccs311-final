extern crate reqwest;
extern crate scraper;

use scraper::{Html, Selector};

// use scraper::Selector;
// use scraper::Html;

// Web_scraping
// website we'll be scraping from - https://www.themoviedb.org/


#[allow(dead_code)]
pub fn web_to_string(url: &str) {
    /* we will be web scraping multiple websites at the same time in parallel */

    let req = reqwest::blocking::get(url).expect("Couldn't load the url"); // error message for when we cannot establish a connection
    // if it's a success you will not see the error message

    let doc_body = Html::parse_document(&req.text().unwrap());

    let review_selector = Selector::parse("div.content p").unwrap();
    // in the selectors we have another opportunity to go parallel
    // every review name starts with "A review by... ". we could iterate through these
    // in parallel to get the information faster
    // this is tough tho sinCe each review needs to be expanded
    // finding the read more button and following the links would be a smarter choice
    // then each link web scraped in parallel

    for review_selector in doc_body.select(&review_selector){
        let review_text = review_selector.text().collect::<Vec<_>>();
        for i in 0..review_text.len() {
            println!("{}", review_text[i]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        //
    }
}
