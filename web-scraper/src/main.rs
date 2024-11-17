use reqwest;
use scraper::{Html, Selector};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create an HTTP client
    let client = reqwest::Client::new();

    // Make a GET request to a website
    let response = client
        .get("https://getdatafor.me/")
        .header("User-Agent", "Mozilla/5.0")
        .send()
        .await?;

    // Get the response text
    let body = response.text().await?;

    // Parse the HTML document
    let document = Html::parse_document(&body);

    // Create a selector for h1 elements
    let h1_selector = Selector::parse("h1").unwrap();
    let p_selector = Selector::parse("p").unwrap();

    // Find and print all h1 elements
    println!("Finding h1 elements:");
    for element in document.select(&h1_selector) {
        println!("- {}", element.text().collect::<Vec<_>>().join(" "));
    }

    // Find and print all paragraph elements
    println!("\nFinding paragraph elements:");
    for element in document.select(&p_selector) {
        println!("- {}", element.text().collect::<Vec<_>>().join(" "));
    }

    Ok(())
}