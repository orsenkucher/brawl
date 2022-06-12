use brawl::bot::Bot;
use brawl::requests::{Request, Requester, ResponseResult};
use brawl::types::Player;
use brawl::RequestError;

#[tokio::main]
async fn main() {
    // let url = Url::parse("https://example.net/#Q0CQ90Lq9").unwrap();
    // println!("{}", url.as_str());
    // return;

    let token = include_str!("token.txt").trim();
    // println!("{}", token);

    // Bot implements `Requester`
    let bot = Bot::new(token);

    let my_tag = "#Q0CQ90Lq9".to_string();

    // run(token).await.unwrap();
    get_player(bot, my_tag).await.unwrap();
}

async fn get_player<R>(bot: R, tag: String) -> ResponseResult<()>
where
    R: Requester,
    RequestError: From<<R as Requester>::Err>, // why do I need this?, <R>
{
    // let resp = bot.get_player(tag).send().await?;
    let resp = bot.get_battlelog(tag).send().await?;
    println!("{:?}", resp);
    Ok::<_, RequestError>(())
}

// async fn run(token: &str) -> Result<(), Box<dyn std::error::Error>> {
//     let url = "https://api.brawlstars.com/v1/players/#Q0CQ90Lq9";
//     const QUERY: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'#').add(b'<').add(b'>');
//     let url = utf8_percent_encode(url, QUERY).to_string();
//     let content: Player = reqwest::Client::builder()
//         .build()?
//         .get(url)
//         .bearer_auth(token)
//         .send()
//         .await?
//         .json()
//         .await?;

//     println!("text: {:?}", content);
//     Ok(())
// }
