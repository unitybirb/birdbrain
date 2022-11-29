mod links;

use std::{env};

use markov_chain::Chain;
use serenity::async_trait;
use serenity::model::prelude::{ChannelId, Ready};
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::model::guild::Member;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{StandardFramework, CommandResult};
use tracing::{info, debug};

#[group]
#[commands(fursona, markov, socials, stream, logs)]
struct General;

struct Handler;

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

    let token = env::var("DISCORD_TOKEN").expect("token"); 
    // let token = "MTA0NjQzNDAwMzAzODc4MTQ3MQ.G9xRga.lt4szJ73LjNp-RZuri3vLnKNwT5ShpAI9FGnqA";
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
