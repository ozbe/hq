use scraper::{Html, Selector};
use std::io::{self, BufRead};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
/// Extract html based on CSS selectors.
///
/// A utility for selecting HTML fragments from an HTML document or fragment with a CSS selector
/// group.
struct Args {
    /// CSS selector group
    ///
    /// Group of CSS Selectors used to select HTML fragments from input HTML.
    selectors: String,
    /// HTML file
    ///
    /// Local file path to a file containing a valid HTML document or fragment. If file argument
    /// missing, hq reads from standard input.
    #[structopt(parse(from_os_str))]
    file: Option<PathBuf>,
}

fn main() {
    let args = Args::from_args();

    let selector = get_selector(&args.selectors);
    let html = get_html(&args.file);
    extract(&selector, html)
}

fn get_selector(selectors: &str) -> Selector {
    Selector::parse(selectors).unwrap()
}

fn get_html(file: &Option<PathBuf>) -> Html {
    let fragment = get_content(file);
    Html::parse_fragment(&fragment)
}

fn get_content(file: &Option<PathBuf>) -> String {
    match file {
        Some(ref file) => read_from_file(file),
        None => read_from_stdin(),
    }
}

fn read_from_file(path: &PathBuf) -> String {
    std::fs::read_to_string(path).unwrap()
}

fn read_from_stdin() -> String {
    let stdin = io::stdin();
    stdin
        .lock()
        .lines()
        .filter_map(|l| l.ok())
        .collect::<Vec<String>>()
        .join("")
}

fn extract(selector: &Selector, html: Html) {
    for element in html.select(&selector) {
        println!("{:?}", element.html());
    }
}
