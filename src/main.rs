mod domains;
mod scrapers;
mod file_ops;

use std::result::Result;
use crate::domains::twitter::Tweet;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://twitter.com/HeathrowNoise";
    let client = reqwest::Client::new();
    let res = client.get(url).send().await?;
    let body = res.text().await?;

    file_ops::write_to_file(&body, "data/body.html")?;

    scrapers::twitter::scrape(&body).iter().flat_map(|tweet| {
        match &tweet {
            t if t.content.contains("westerly") => Some(Tweet::new(t.id, t.date_time, "westerly".to_string())),
            t if t.content.contains("easterly") => Some(Tweet::new(t.id, t.date_time, "easterly".to_string())),
            _ => None,
        }
    }).for_each(|tweet|
        println!("{} >> {} ({}): {}",
                 tweet.id,
                 tweet.date_time.format("%Y-%m-%d %H:%M:%S"),
                 tweet.date_time.timestamp_millis(),
                 tweet.content
        )
    );
    Ok(())
}
