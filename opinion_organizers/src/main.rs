use crate::sentiment_analysis::get_sentiment_counts;
use crate::web_scraper::review_collection;

mod web_scraper;
mod sentiment_analysis;

fn analyze_review(url: &str) -> f64 {
    use rayon::iter::*;
    
    println!("{:?}", review_collection(url));  // replace test_arr

    let test_arr = vec![  // replace with web_scraper output
        "It's a contemplative adventure and an emotional exploration that captivated me from its opening moments...",
        "While not all-together perfect, the film represents a monumental cinematic achievement that deserves to be placed high within the caliber of Nolan's filmography.",
        "As spectacular as it is flawed."
    ];

    let (pos, neg) = test_arr
        .par_iter()
        .map(|&s| get_sentiment_counts(s.to_string()))
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
