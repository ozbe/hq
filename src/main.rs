use scraper::{Html, Selector};
use std::io::{self, BufRead};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Args {
    selectors: String,
}

fn main() {
    let args = Args::from_args();
    let selector = Selector::parse(&args.selectors).unwrap();

    let content: String = {
        let stdin = io::stdin();
        stdin
            .lock()
            .lines()
            .filter_map(|l| l.ok())
            .collect::<Vec<String>>()
            .join("")
    };

    let document = Html::parse_fragment(&content);
    for element in document.select(&selector) {
        println!("{:?}", element.html());
    }
}
