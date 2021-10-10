use serde_derive::Serialize;
use crate::domains::twitter::Tweet;

#[derive(Debug, Clone, Serialize)]
pub struct PlaneNoise {
    pub id: u64,
    pub epoch: i64,
    pub date_time: String,
    pub direction: String,
    pub tweet_content: String,
}

impl PlaneNoise {
    fn new(tweet: &Tweet, direction: String) -> PlaneNoise {
        PlaneNoise { id: tweet.id, epoch: tweet.epoch, date_time: tweet.date_time.to_string(), direction, tweet_content: tweet.content.to_string()}
    }

    pub fn from(tweets: &Vec<Tweet>) -> Vec<PlaneNoise> {
        tweets.iter().flat_map(|tweet| {
            match &tweet.content {
                t if t.contains("westerly") => Some(PlaneNoise::new(tweet, "westerly".to_string())),
                t if t.contains("easterly") => Some(PlaneNoise::new(tweet, "easterly".to_string())),
                _ => None,
            }
        }).collect::<Vec<PlaneNoise>>()
    }
}
