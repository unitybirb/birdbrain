mod links;
use std::{env};

use markov_chain::Chain;
use rand::prelude::*;
use reqwest::Error;
use reqwest::header::USER_AGENT;
use serde::Deserialize;
use serenity::async_trait;
use serenity::model::prelude::{ChannelId, Ready};
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::model::guild::Member;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{StandardFramework, CommandResult};
use tracing::{info, debug};

#[group]
#[commands(fursona, markov, socials, stream, logs, derpi)]
struct General;

struct Handler;

#[derive(Deserialize)]
struct e621_object {
    
}
#[derive(Deserialize, Debug)]
struct DerpiObject {
    id: i32,
    score: i32,
    faves: i32,
    downvotes: i32,
    description: String,
    tags: Vec<String>,
    view_url: String
}

#[derive(Deserialize, Debug)]
struct DerpiResponse {
    images: Vec<DerpiObject>
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "h" && !msg.is_own(&ctx) {
            msg.channel_id.say(&ctx, "h").await.expect("Failed while sending message");
        }
    }

    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {
            new_member.guild_id.channels(&ctx)
            .await
            .expect("Failed while receiving list of channels")
            .get(&ChannelId(998225684612796526))
            .expect("Failed while getting welcome channel")
            .say(&ctx, format!("Welcome to the cum zone {}", new_member.nick.expect("Failed while getting username")))
            .await
            .expect("Failed while sending welcome message");
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .group(&GENERAL_GROUP);

    // let token = env::var("DISCORD_TOKEN").expect("token"); 
    let token = "MTA0NjQzNDAwMzAzODc4MTQ3MQ.G9xRga.lt4szJ73LjNp-RZuri3vLnKNwT5ShpAI9FGnqA";
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT | GatewayIntents::GUILDS | GatewayIntents::GUILD_MEMBERS;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn fursona(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Check out my fursona here: https://ref.birdtech.dev").await?;
    Ok(())
}

#[command]
async fn markov(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "cum").await?;
    let mut chain:Chain<&str> = Chain::new(3);
    chain.train(vec![]);
    Ok(())
}

#[command]
async fn socials (ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, links::get_link_string()).await?;
    Ok(())
}

#[command]
async fn stream (ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "My stream can be found at https://www.twitch.tv/unitybirb").await?;
    Ok(())
}

#[command]
async fn logs (ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "https://www.fflogs.com/character/id/13439791").await?;
    Ok(())
}


#[command]
async fn derpi (ctx: &Context, msg: &Message) -> CommandResult {
    let client = reqwest::Client::new();
    let querystring = &msg.content[6..];
    let url = format!("https://derpibooru.org/api/v1/json/search/images?q={query}&filter_id={filter}", query = &querystring, filter = 37432);
    let response = client.get(&url).header(USER_AGENT, "Birdbrain").send().await.expect("Couldn't get response");
    let derpi_object_list: Result<DerpiResponse, Error> = response.json().await;
    let deserialized = match &derpi_object_list {
        Ok(object_list) => &object_list.images,
        Err(error) =>  panic!("Deserialization failed with error {}", &error)
    };
    let received_picture = &deserialized.get(rand::thread_rng().gen_range(0..deserialized.len())).expect("Couldn't receive picture");
    let artist = &received_picture.tags.iter().find(|x| x.starts_with("artist:")).expect("Couldn't extract artist")[7..];
    msg.reply(ctx, format!("Found image {} by {}. Its score is {} with {} downvotes and {} faves.\n{}",
        &received_picture.id, &artist, &received_picture.score, &received_picture.downvotes, &received_picture.faves, &received_picture.view_url)).await?;
    msg.reply(ctx, format!("Description: {}", &received_picture.description[..1987])).await.expect("Description too long");
    Ok(())
}
