use anyhow::{Context, Result};
use cssparser::{BasicParseErrorKind, ParseErrorKind};
use scraper::{Html, Selector};
use selectors::parser::SelectorParseErrorKind;
use std::error::Error;
use std::fmt::{Display, Formatter};
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

fn main() -> Result<()> {
    let args = Args::from_args();

    let selector = get_selector(&args.selectors)?;
    let html = get_html(&args.file)?;
    extract(&selector, html);

    Ok(())
}

fn get_selector(selectors: &str) -> Result<Selector> {
    Selector::parse(selectors)
        .map_err(SelectorParseError::from)
        .with_context(|| format!("could not parse selectors `{}`", selectors))
        .map_err(From::from)
}

fn get_html(file: &Option<PathBuf>) -> Result<Html> {
    let fragment = get_content(file)?;
    Ok(Html::parse_fragment(&fragment))
}

fn get_content(file: &Option<PathBuf>) -> Result<String> {
    match file {
        Some(ref file) => read_from_file(file),
        None => Ok(read_from_stdin()),
    }
}

fn read_from_file(path: &PathBuf) -> Result<String> {
    std::fs::read_to_string(path)
        .with_context(|| format!("could not read file `{}`", path.as_path().display()))
        .map_err(From::from)
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

#[derive(Debug)]
struct SelectorParseError(String);

impl Display for SelectorParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<'i> From<cssparser::ParseError<'i, SelectorParseErrorKind<'i>>> for SelectorParseError {
    fn from(e: cssparser::ParseError<'i, SelectorParseErrorKind<'i>>) -> Self {
        let msg = match e.kind {
            ParseErrorKind::Basic(b) => match b {
                BasicParseErrorKind::UnexpectedToken(_) => "unexpected token".to_string(),
                BasicParseErrorKind::EndOfInput => {
                    "end of the input was encountered unexpectedly.".to_string()
                }
                BasicParseErrorKind::AtRuleInvalid(a) => format!("invalid `@` rule {}", a),
                BasicParseErrorKind::AtRuleBodyInvalid => {
                    "body of an '@' rule was invalid".to_string()
                }
                BasicParseErrorKind::QualifiedRuleInvalid => {
                    "qualified rule was encountered that was invalid".to_string()
                }
            },
            ParseErrorKind::Custom(kind) => match kind {
                SelectorParseErrorKind::BadValueInAttr(_) => "bad value in attribute".to_string(),
                SelectorParseErrorKind::PseudoElementInComplexSelector => {
                    "pseudo element is complex selector".to_string()
                }
                SelectorParseErrorKind::NoQualifiedNameInAttributeSelector(_) => {
                    "no qualified name in attribute selector".to_string()
                }
                SelectorParseErrorKind::EmptySelector => "empty selector".to_string(),
                SelectorParseErrorKind::DanglingCombinator => "dangling combinator".to_string(),
                SelectorParseErrorKind::NonSimpleSelectorInNegation => {
                    "non simple selector in negation".to_string()
                }
                SelectorParseErrorKind::NonCompoundSelector => "non compound selector".to_string(),
                SelectorParseErrorKind::NonPseudoElementAfterSlotted => {
                    "non pseudo element after slotted".to_string()
                }
                SelectorParseErrorKind::InvalidPseudoElementAfterSlotted => {
                    "invalid pseudo element after slotted".to_string()
                }
                SelectorParseErrorKind::UnexpectedTokenInAttributeSelector(_) => {
                    "unexpected token in attribute selector".to_string()
                }
                SelectorParseErrorKind::PseudoElementExpectedColon(_) => {
                    "pseudo element expected colon".to_string()
                }
                SelectorParseErrorKind::PseudoElementExpectedIdent(_) => {
                    "pseudo element expected identity".to_string()
                }
                SelectorParseErrorKind::NoIdentForPseudo(_) => "no identity for pseudo".to_string(),
                SelectorParseErrorKind::UnsupportedPseudoClassOrElement(s) => {
                    format!("unsupported pseudo class or element `{}`", s)
                }
                SelectorParseErrorKind::UnexpectedIdent(s) => {
                    format!("unexpected identity `{}`", s)
                }
                SelectorParseErrorKind::ExpectedNamespace(s) => {
                    format!("expected namespace `{}`", s)
                }
                SelectorParseErrorKind::ExpectedBarInAttr(_) => {
                    "expected bar in attribute".to_string()
                }
                SelectorParseErrorKind::InvalidQualNameInAttr(_) => {
                    "invalid qualified name in attribute".to_string()
                }
                SelectorParseErrorKind::ExplicitNamespaceUnexpectedToken(_) => {
                    "explicit namespace unexpected token".to_string()
                }
                SelectorParseErrorKind::ClassNeedsIdent(_) => "class needs identity".to_string(),
                SelectorParseErrorKind::EmptyNegation => "empty negation".to_string(),
            },
        };
        SelectorParseError(msg)
    }
}

impl Error for SelectorParseError {}
