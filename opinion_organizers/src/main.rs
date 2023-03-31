use crate::sentiment_analysis::get_sentiment_counts;
use crate::web_scraper::{review_collection, web_to_string};
mod web_scraper;
mod sentiment_analysis;

fn analyze_review(url: &str) -> f64 {
    use rayon::iter::*;

    let reviews = review_collection(url);

    let (pos, neg) = reviews
        .par_iter()
        .map(|s| get_sentiment_counts(s.to_string()))
        .reduce(|| (0, 0), |(pos_count1, neg_count1), (pos_count2, neg_count2)| { (pos_count1 + pos_count2, neg_count1 + neg_count2) } );

    if pos == 0 { return neg as f64; }
    else if neg == 0 { return pos as f64 }
    else { return pos as f64 / neg as f64 }
}

fn main() {
    use std::io;

    loop {
        println!("Please enter a link:");

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let link = input.trim();
                let ratio = analyze_review(link);

                println!("The score of these reviews is {}.", ratio);
                if ratio > 0.7 { println!("You can enjoy this one!"); }
                else { println!("You might want to think twice about this..."); }
                    
                break;
            }
            Err(error) => {
                println!("Error reading input: {}", error);
                continue;
            }
        }
    }
}
