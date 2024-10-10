use reqwest;
use dotenv::dotenv;
use twitter_v2::TwitterApi;
use twitter_v2::authorization::{Oauth2Token, BearerToken};
use twitter_v2::query::{TweetField, UserField};

async fn env_token_connection(_token: &str) -> Result<BearerToken, Box<dyn std::error::Error>> {
    dotenv().ok();
    let x_bearer_token = std::env::var("TWITTER_BEARER_TOKEN")?;
    let auth = BearerToken::new(x_bearer_token);
    Ok(auth)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let auth = env_token_connection("TWITTER_BEARER_TOKEN").await?;

    let tweet = TwitterApi::new(auth)
        .get_tweet(1843837033627918740)
        .send()
        .await?
        .into_data()
        .expect("this tweet should exist");

    println!("{:#?}", tweet);

    Ok(())
}
