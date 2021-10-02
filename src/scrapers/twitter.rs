use std::convert::TryFrom;
use chrono::{DateTime, NaiveDateTime, Utc};
use scraper::{Html, Selector};
use crate::domains::twitter::Tweet;

pub fn scrape(body: &str) -> Vec<Tweet> {
    let html_body = Html::parse_fragment(body);
    let content_selector = Selector::parse("div.content").unwrap();
    let tweet_contents_selector = Selector::parse("p.tweet-text").unwrap();
    let date_time_selector = Selector::parse("span._timestamp").unwrap();
    let tweet_id_selector = Selector::parse("a.tweet-timestamp").unwrap();

    // let i = for element in html_body.select(&content_selector) {
    html_body.select(&content_selector).map(|element| {
        let tweet = element.select(&tweet_contents_selector).next().unwrap().inner_html();
        let tweet_id = element.select(&tweet_id_selector).next().unwrap().value().attr("href").unwrap().split("/status/").last().unwrap().parse::<u64>().unwrap();
        let date_time = element.select(&date_time_selector).last().unwrap().value().attr("data-time-ms").unwrap().to_string().parse::<i64>().unwrap();
        let n_date_time = NaiveDateTime::from_timestamp(date_time/1000, u32::try_from(date_time % 1000).unwrap());
        let time = DateTime::<Utc>::from_utc(n_date_time, Utc);
        Tweet::new(tweet_id, time, tweet)
    }).collect::<Vec<Tweet>>()
}