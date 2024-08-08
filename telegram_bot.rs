use std::env;
use teloxide::{prelude::*, utils::command::BotCommands};

#[derive(BotCommands, Clone)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "create a new study group.")]
    NewGroup(String),
    #[command(description = "add participant to a study group.")]
    AddParticipant(String, String),
}

async fn answer(
    cx: UpdateWithCx<AutoSend<Bot>, Message>,
    command: Command,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    match command {
        Command::Help => {
            if let Err(e) = cx.answer(Command::descriptions().to_string()).send().await {
                log::error!("Failed to send help message: {:?}", e);
            }
        }
        Command::NewGroup(group_name) => {
            if let Err(e) = cx.answer(format!("New group '{}' created.", group_name)).send().await {
                log::error!("Failed to create new group '{}': {:?}", group_name, e);
            }
        }
        Command::AddParticipant(group_name, participant) => {
            if let Err(e) = cx
                .answer(format!(
                    "Participant '{}' added to group '{}'.",
                    participant, group_name
                ))
                .send().await
            {
                log::error!(
                    "Failed to add participant '{}' to group '{}': {:?}",
                    participant,
                    group_name,
                    e
                );
            }
        }
    }
    Ok(())
}

async fn handle_message(
    bot: AutoSend<Bot>,
    message: Message,
    command: Command,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if let Err(e) = answer(UpdateWithCx { bot, update: message }, command).await {
        log::error!("Error handling message: {:?}", e);
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    teloxide::enable_logging!();
    log::info!("Starting bot...");

    let bot = Bot::from_env().auto_send();

    let bot_name: String = env::var("BOT_NAME").expect("BOT_NAME must be set");

    if let Err(e) = teloxide::commands_repl(bot, bot_name, handle_message).await {
        log::error!("Bot encountered an error: {}", e);
    }
}