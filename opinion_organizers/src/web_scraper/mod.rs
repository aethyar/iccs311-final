extern crate reqwest;
extern crate scraper;
extern crate rayon;

use scraper::{Html, Selector};
use rayon::prelude::*;

// use scraper::Selector;
// use scraper::Html;

// Web_scraping
// we'll be scraping from the individual reviews on - https://www.themoviedb.org/ only.


#[allow(dead_code)]
pub fn web_to_string(urls: &[&str]) -> Vec<String> {    // we take in a Vec of links that can be variale in amount

    urls.par_iter().map(|url| {
        
        let req = reqwest::blocking::get(*url).unwrap();     // .expect("Couldn't load the url"); // error message for when we cannot establish a connection
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        //
    }
}
