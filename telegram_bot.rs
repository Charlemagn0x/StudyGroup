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
    AddParticipant(String, String), // group_name, participant
}

async fn answer(
    cx: UpdateWithCx<AutoSend<Bot>, Message>,
    command: Command,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    match command {
        Command::Help => {
            cx.answer(Command::descriptions().to_string()).send().await?;
        }
        Command::NewGroup(group_name) => {
            // Handle creating a new group (example implementation)
            cx.answer(format!("New group '{}' created.", group_name))
                .send()
                .await?;
        }
        Command::AddParticipant(group_name, participant) => {
            // Handle adding a participant to a group (example implementation)
            cx.answer(format!(
                "Participant '{}' added to group '{}'.",
                participant, group_name
            ))
            .send()
                .await?;
        }
    }

    Ok(())
}

async fn handle_message(
    bot: AutoSend<Bot>,
    message: Message,
    command: Command,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    answer(UpdateWithCx { bot, update: message }, command).await
}

#[tokio::main]
async fn main() {
    teloxide::enable_logging!();
    log::info!("Starting bot...");

    let bot = Bot::from_env().auto_send();

    let bot_name: String = env::var("BOT_NAME").expect("BOT_NAME must be set");

    teloxide::commands_repl(bot, bot_name, handle_message).await;
}