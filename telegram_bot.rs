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

async fn send_response(
    update_context: UpdateWithCx<AutoSend<Bot>, Message>,
    command: Command,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    match command {
        Command::Help => {
            if let Err(error) = update_context.answer(Command::descriptions().to_string()).send().await {
                log::error!("Failed to send help message: {:?}", error);
            }
        }
        Command::NewGroup(group_name) => {
            if let Err(error) = update_context.answer(format!("New group '{}' created.", group_name)).send().await {
                log::error!("Failed to create new group '{}': {:?}", group_name, error);
            }
        }
        Command::AddParticipant(group_name, participant_name) => {
            if let Err(error) = update_context
                .answer(format!(
                    "Participant '{}' added to group '{}'.",
                    participant_name, group_name
                ))
                .send().await
            {
                log::error!(
                    "Failed to add participant '{}' to group '{}': {:?}",
                    participant_name,
                    group_name,
                    error
                );
            }
        }
    }
    Ok(())
}

async fn process_message_command(
    bot_client: AutoSend<Bot>,
    received_message: Message,
    parsed_command: Command,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if let Err(error) = send_response(UpdateWithCx { bot: bot_client, update: received_message }, parsed_command).await {
        log::error!("Error processing message command: {:?}", error);
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    teloxide::enable_logging!();
    log::info!("Bot is starting...");

    let bot_client = Bot::from_env().auto_send();

    let bot_name: String = env::var("BOT_NAME").expect("Environment variable BOT_NAME must be set");

    if let Err(error) = teloxide::commands_repl(bot_client, bot_name, process_message_command).await {
        log::error!("Bot encountered an error: {}", error);
    }
}