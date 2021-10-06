mod domains;
mod scrapers;
mod file_ops;
mod formats;

use std::result::Result;
use structopt::StructOpt;
use crate::domains::plane_noise::PlaneNoise;

#[derive(StructOpt)]
#[structopt(rename_all = "kebab-case")]
struct Opt {
    #[structopt(short = "f",
    long = "format",
    default_value = "json",
    possible_values = &vec![ "bitbar", "json" ],
    help = "The format for the output")]
    format: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Opt::from_args();
    let format: String = opts.format;

    let url = "https://twitter.com/HeathrowNoise";
    let client = reqwest::Client::new();
    let res = client.get(url).send().await?;
    let body = res.text().await?;

    // file_ops::write_to_file(&body, "body.html")?;

    let data = PlaneNoise::from(&scrapers::twitter::scrape(&body));

    match format.as_str() {
        "bitbar" => formats::bitbar::display(&data),
        _ => formats::json::display(&data), // json is default format
    }

    Ok(())
}
