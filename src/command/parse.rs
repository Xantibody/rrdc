use rrdc::service::html_parser::HtmlParser;
use rrdc::Result;
use std::io::{self, Read};

pub async fn execute() -> Result<()> {
    // Read HTML from stdin
    let mut html = String::new();
    io::stdin().read_to_string(&mut html)?;

    // Parse HTML
    let parser = HtmlParser::new();
    let releases = parser.parse(&html)?;

    // Output JSON to stdout
    let json = serde_json::to_string_pretty(&releases)?;
    println!("{}", json);

    Ok(())
}
