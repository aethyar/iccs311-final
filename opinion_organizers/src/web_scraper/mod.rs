extern crate reqwest;
extern crate scraper;

use scraper::{Html, Selector};

// use scraper::Selector;
// use scraper::Html;

// Web_scraping
// website we'll be scraping from - https://www.themoviedb.org/


#[allow(dead_code)]
pub fn web_to_string(url: &str) -> String {
    let req = reqwest::blocking::get(url).expect("Couldn't load the url"); // error message for when we cannot establish a connection
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        //
    }
}
