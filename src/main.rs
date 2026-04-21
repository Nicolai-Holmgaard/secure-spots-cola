use crate::api::client::{SaleRequest, get_member_id, get_member_info, post_sale};
use clap::Parser;
use serde::{Deserialize, Serialize};
mod api;

#[derive(Serialize, Deserialize)]
struct SSCConfig {
    username: String,
    room: i32,
    url: String,
}

impl ::std::default::Default for SSCConfig {
    fn default() -> Self {
        Self {
            username: "".into(),
            room: 10,
            url: "http://stregsystem.fklub.dk".into(),
        }
    }
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short)]
    username: Option<String>,
    buystring: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), confy::ConfyError> {
    let cli = Cli::parse();
    let cfg: SSCConfig = confy::load("secure-sports-cola", "config")?;

    //println!("name: {:?}", cli.username);
    //println!("buystring: {:?}", cli.buystring.join(" "));
    //println!("url: {:?}", cfg.url);
    let mut username = cli.username.unwrap_or(cfg.username);
    let member_id = get_member_id(&cfg.url, &username).await.unwrap();
    //println!("member_id: {member_id}");
    //get_member_info(&cfg.url, &member_id).await;

    post_sale(
        &cfg.url,
        SaleRequest {
            member_id: member_id,
            room: cfg.room,
            buystring: format!("{} {}", &username, cli.buystring.join(" ")),
        },
    )
    .await
    .unwrap();
    Ok(())
}
