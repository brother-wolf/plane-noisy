use std::result::Result;

use scrapers_lib::Site;
use scrapers_lib::twitter::Twitter;
use structopt::StructOpt;

use crate::domains::plane_noise::PlaneNoise;

mod domains;
mod formats;

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

    let data = PlaneNoise::from(&Twitter::scrape(&body));

    match format.as_str() {
        "bitbar" => formats::bitbar::display(&data),
        _ => formats::json::display(&data), // json is default format
    }

    Ok(())
}
