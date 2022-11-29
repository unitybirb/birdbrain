mod links;
use std::{cmp, env};

use markov_chain::Chain;
use rand::prelude::*;
use reqwest::header::USER_AGENT;
use reqwest::Error;
use serde::Deserialize;
use serenity::async_trait;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{CommandResult, StandardFramework};
use serenity::model::channel::Message;
use serenity::model::guild::Member;
use serenity::model::prelude::{GuildId, Ready};
use serenity::model::user::User;
use serenity::prelude::*;

use crate::links::Socials;

#[group]
#[commands(fursona, socials, stream, logs, derpi, e621, help)]
struct General;

struct Handler;

#[derive(Deserialize)]
struct E621Object {
    id: u32,
    file: E621File,
    score: E621Score,
    tags: E621Tags,
    description: String,
    fav_count: u32,
}

#[derive(Deserialize)]
struct E621Response {
    posts: Vec<E621Object>,
}

#[derive(Deserialize)]
struct E621Tags {
    general: Vec<String>,
    species: Vec<String>,
    artist: Vec<String>,
}

#[derive(Deserialize)]
struct E621File {
    url: Option<String>,
}

#[derive(Deserialize)]
struct E621Score {
    down: i32,
    total: u32,
}

#[derive(Deserialize)]
struct DerpiObject {
    id: u32,
    score: i32,
    faves: u32,
    downvotes: i32,
    description: String,
    tags: Vec<String>,
    view_url: String,
}

#[derive(Deserialize)]
struct DerpiResponse {
    images: Vec<DerpiObject>,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "h" && !msg.is_own(&ctx) {
            msg.channel_id
                .say(&ctx, "h")
                .await
                .expect("Failed while sending message");
        }
    }

    async fn guild_member_removal(
        &self,
        ctx: Context,
        guild_id: GuildId,
        user: User,
        _member_data_if_available: Option<Member>,
    ) {
        let guild = guild_id
            .to_partial_guild(&ctx)
            .await
            .expect("Failed while getting guild");
        let system_channel_id = guild
            .system_channel_id
            .expect("Failed while getting system channel id");
        guild
            .channels(&ctx)
            .await
            .expect("Failed while getting guild channels")
            .get(&system_channel_id)
            .expect("Failed while getting welcome channel")
            .say(
                &ctx,
                format!(
                    "**[LEAVE]** {} has been found dead behind a dumpster",
                    user.name
                ),
            )
            .await
            .expect("Failed while sending leave message");
    }

    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {
        let guild = new_member
            .guild_id
            .to_partial_guild(&ctx)
            .await
            .expect("Failed while getting guild");
        let system_channel_id = guild
            .system_channel_id
            .expect("Failed while getting system channel id");
        guild
            .channels(&ctx)
            .await
            .expect("Failed while getting guild channels")
            .get(&system_channel_id)
            .expect("Failed while getting welcome channel")
            .say(
                &ctx,
                format!(
                    "Welcome to the cum zone {}",
                    new_member.nick.expect("Failed while getting username")
                ),
            )
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

    let token = env::var("DISCORD_TOKEN").expect("token");
    let intents = GatewayIntents::non_privileged()
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MEMBERS;
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
    msg.reply(ctx, "Check out my fursona here: https://ref.birdtech.dev")
        .await?;
    Ok(())
}

/* TODO */
#[command]
async fn markov(ctx: &Context, msg: &Message) -> CommandResult {
    let mut generated_text = String::new();
    let mut chain: Chain<&str> = Chain::new(3);
    chain.train(vec![]);
    chain
        .generate()
        .iter()
        .for_each(|generated| generated_text.push_str(&format!("{} ", generated)));
    msg.reply(&ctx, generated_text)
        .await
        .expect("Couldn't send message");
    Ok(())
}

#[command]
async fn socials(ctx: &Context, msg: &Message) -> CommandResult {
    let mut socials = Socials {
        social_vec: vec![
            ("Mastodon", "https://tech.lgbt/@bird"),
            ("Twitter", "https://twitter.com/unitybirb"),
            ("Tumblr", "https://unity-birdposts.tumblr.com"),
            ("Cohost", "https://cohost.org/unitybirb"),
        ],
    };
    msg.reply(ctx, socials.get_link_string()).await?;
    Ok(())
}

#[command]
async fn stream(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(
        ctx,
        "My stream can be found at https://www.twitch.tv/unitybirb",
    )
    .await?;
    Ok(())
}

#[command]
async fn logs(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "https://www.fflogs.com/character/id/13439791")
        .await?;
    Ok(())
}

#[command]
async fn derpi(ctx: &Context, msg: &Message) -> CommandResult {
    let client = reqwest::Client::new();
    let querystring = &msg.content[6..];
    let url = format!(
        "https://derpibooru.org/api/v1/json/search/images?q={query}&filter_id={filter}",
        query = &querystring,
        filter = 37432
    );
    let response = client
        .get(&url)
        .header(USER_AGENT, "Birdbrain")
        .send()
        .await
        .expect("Couldn't get response");
    let derpi_object_list: Result<DerpiResponse, Error> = response.json().await;
    let deserialized = match &derpi_object_list {
        Ok(object_list) => match object_list.images.len() {
            0 => {
                msg.reply(
                    &ctx,
                    "Couldn't find any images! Are you sure the tags you're looking for exist?",
                )
                .await
                .expect("Couldn't send error message");
                return Ok(());
            }
            _ => &object_list.images,
        },
        Err(error) => panic!("Deserialization failed with error {}", &error),
    };
    let received_picture = &deserialized
        .get(get_random_number(deserialized.len()))
        .expect("Couldn't get image");
    let artist = &received_picture
        .tags
        .iter()
        .find(|x| x.starts_with("artist:"))
        .expect("Couldn't extract artist")[7..];
    msg.reply(
        ctx,
        format!(
            "Found image {} by {}. Its score is {} with {} downvotes and {} faves.\n{}",
            &received_picture.id,
            &artist,
            &received_picture.score,
            &received_picture.downvotes,
            &received_picture.faves,
            &received_picture.view_url
        ),
    )
    .await
    .expect("Couldn't post image");
    msg.reply(
        ctx,
        format!(
            "Description: {}",
            &received_picture.description[..get_description_max(&received_picture.description)]
        ),
    )
    .await
    .expect("Couldn't post message");
    Ok(())
}

#[command]
async fn e621(ctx: &Context, msg: &Message) -> CommandResult {
    let client = reqwest::Client::new();
    let querystring = &msg.content[5..];
    let url = format!(
        "https://e621.net/posts.json?tags={query}",
        query = &querystring
    );
    let response = client
        .get(&url)
        .header(USER_AGENT, "Birdbrain")
        .send()
        .await
        .expect("Couldn't get response");
    let e621_object_list: Result<E621Response, Error> = response.json().await;
    let deserialized = match &e621_object_list {
        Ok(object_list) => match object_list.posts.len() {
            0 => {
                msg.reply(
                    &ctx,
                    "Couldn't find any images! Are you sure the tags you're looking for exist?",
                )
                .await
                .expect("Couldn't send error message");
                return Ok(());
            }
            _ => &object_list.posts,
        },
        Err(error) => panic!("Deserialization failed with error {}", &error),
    };
    let filtered: Vec<&E621Object> = deserialized
        .iter()
        .filter(|post| post.file.url.is_some())
        .collect();
    let received_picture = filtered
        .get(get_random_number(deserialized.len()))
        .expect("Couldn't get image");
    let artists = &received_picture.tags.artist;
    let artist = match &artists.len() {
        0 => String::from("unknown artist"),
        _ => artists.get(0).unwrap().to_owned(),
    };
    let description = &received_picture.description;
    let url = received_picture.file.url.as_ref().unwrap();
    msg.reply(
        ctx,
        format!(
            "Found image {} by {}. Its score is {} with {} downvotes and {} faves.\n{}",
            &received_picture.id,
            artist,
            &received_picture.score.total,
            &received_picture.score.down,
            &received_picture.fav_count,
            url
        ),
    )
    .await
    .expect("Couldn't post image");
    msg.reply(
        ctx,
        format!(
            "Description: {}",
            &description[..get_description_max(description)]
        ),
    )
    .await
    .expect("Couldn't post message");
    Ok(())
}

#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    let response = match &msg.content[cmp::min(6, msg.content.len())..] {
        "socials" => "Posts all of my available social media",
        "fursona" => "Posts a link to my fursona reference",
        "stream" => "Posts a link to my stream",
        "logs" => "Posts a link to my FFLogs page",
        "e621" => "Queries e621 with your given tags. Usage: !e621 [tag1] [tag2]",
        "derpi" => "Queries Derpibooru with your given tags. Usage: !derpi [tag1], [tag2]",
        _ => "The following commands are available: !socials, !fursona, !stream, !logs, !e621 and !derpi"
    };
    msg.reply(ctx, response)
        .await
        .expect("Cound't post message");
    Ok(())
}

fn get_random_number(max: usize) -> usize {
    rand::thread_rng().gen_range(0..max)
}

fn get_description_max(description: &str) -> usize {
    if description.len() > 1987 {
        1987
    } else {
        description.len()
    }
}
