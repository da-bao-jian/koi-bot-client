use crate::consts::{BUY, CLOSE, MAIN_MENU};
use crate::handlers::callback_handlers::{
    handle_buy_amount_callback, handle_buy_callback, handle_buy_token_callback,
    handle_close_callback, handle_menu_callback, handle_private_tx_callback,
    handle_rebate_callback, handle_receive_token_callback, handle_send_tx_callback,
    handle_wallet_callback,
};
use crate::handlers::dialogue_handlers::{
    buy_address_dialogue_handler, buy_address_or_token_handler, buy_amount_dialogue_handler,
    PromptDialogueState,
};
use crate::handlers::{delete_previous_messages, matching_sub_menu, SubMenuType};
use crate::keyboards::buy_buttons::BuyButtons;
use crate::keyboards::menu_keyboard;
use crate::requests::on_chain;
use crate::storages::{TgMessage, TgMessageStorage, GLOBAL_MAIN_MENU_STORAGE};
use std::sync::Arc;
use teloxide::dispatching::HandlerExt;
use teloxide::{
    dispatching::{dialogue::InMemStorage, UpdateFilterExt},
    dptree,
    error_handlers::LoggingErrorHandler,
    payloads::SendMessageSetters,
    prelude::{Dispatcher, Requester},
    types::{CallbackQuery, Message, ParseMode, Update},
    utils::command::BotCommands,
    Bot,
};
use tokio::time::{sleep, Duration};

use std::fmt;
use teloxide::dispatching::dialogue::InMemStorageError;

#[derive(Debug)]
#[allow(dead_code)]
pub enum TgError {
    AnyhowError(anyhow::Error),
    Parse(String),
    TeloxideRequest(teloxide::RequestError),
    TeloxideInMemStorageError(InMemStorageError),
    UnmatchedQuery(teloxide::types::CallbackQuery),
    NoQueryData(teloxide::types::CallbackQuery),
    NoQueryMessage(teloxide::types::CallbackQuery),
    UserNotFound(teloxide::types::Message),
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Supported commands:")]
enum Command {
    #[command(description = "Show Available Commands")]
    Help,
    #[command(description = "Main Menu")]
    Menu,
    #[command(description = "Display all wallet addresses")]
    Wallets,
    #[command(description = "Start the bot")]
    Start,
    #[command(description = "Display Trade History")]
    History,
}

#[derive(Clone, Debug)]
pub struct TgBot {
    bot: Bot,
}

impl TgBot {
    pub fn new() -> Self {
        let bot = Bot::from_env();
        Self { bot }
    }

    pub async fn init(self) -> Result<(), TgError> {
        let handler = dptree::entry()
            .branch(Update::filter_message().filter_command::<Command>().endpoint(command_callback))
            .branch(Update::filter_callback_query().endpoint(button_callback))
            .branch(
                 Update::filter_message()
                     .enter_dialogue::<Message,InMemStorage<PromptDialogueState>,PromptDialogueState>()
                         .branch(dptree::case![PromptDialogueState::BuyStartAddressPrompt]
                             .endpoint(buy_address_dialogue_handler))
                         .branch(dptree::case![PromptDialogueState::BuyAddressReceived]
                             .endpoint(buy_address_or_token_handler))
                         .branch(dptree::case![PromptDialogueState::ReceiveStartAddressPrompt]
                             .endpoint(buy_address_dialogue_handler))
                         .branch(dptree::case![PromptDialogueState::ReceiveAddressReceived]
                             .endpoint(buy_address_or_token_handler))
                         .branch(dptree::case![PromptDialogueState::BuyAmountReceived]
                             .endpoint(buy_amount_dialogue_handler))
            );

        Dispatcher::builder(self.bot, handler)
            .error_handler(LoggingErrorHandler::with_custom_text(
                "An error has occurred in the dispatcher",
            ))
            .dependencies(dptree::deps![InMemStorage::<PromptDialogueState>::new()])
            .enable_ctrlc_handler()
            .build()
            .dispatch()
            .await;
        Ok(())
    }
}

async fn command_callback(bot: Bot, cmd: Command, msg: Message) -> Result<(), TgError> {
    match cmd {
        Command::Help => {
            let _ = bot
                .send_message(msg.chat.id, Command::descriptions().to_string())
                .await?;
        }
        Command::Menu => {
            let keyboard = menu_keyboard();
            let menu_msg = on_chain::get_on_chain_info().await?;

            // send the new message
            let message_sent = bot
                .send_message(msg.chat.id, menu_msg)
                .parse_mode(ParseMode::MarkdownV2)
                .reply_markup(keyboard)
                .await?;
            let message_sent = Arc::new(message_sent);

            // Updates the GLOBAL_MAIN_MENU_STORAGE
            let _user_name = message_sent
                .clone()
                .from()
                .and_then(|user| user.username.as_ref())
                .and_then(|user_name| {
                    let message = TgMessage {
                        chat_id: message_sent.chat.id,
                        message_id: message_sent.id,
                        message: message_sent.clone(),
                    };
                    GLOBAL_MAIN_MENU_STORAGE.insert(user_name.to_string(), message);
                    Some(user_name)
                });

            // delete previous messages
            let last_message_id = message_sent.id;
            let _ =
                delete_previous_messages(&bot, msg.chat.id.0, last_message_id.0 - 1, 20).await?;
        }
        Command::Start => {
            sleep(Duration::from_secs(3)).await;
            let keyboard = menu_keyboard();
            let menu_msg = on_chain::get_on_chain_info_start().await?;

            // send the new message
            let _message_sent = bot
                .send_message(msg.chat.id, menu_msg)
                .parse_mode(ParseMode::MarkdownV2)
                .reply_markup(keyboard)
                .await?;
        }
        Command::Wallets => {
            todo!()
        }
        Command::History => {
            todo!()
        }
    }
    Ok(())
}

async fn button_callback(
    bot: Bot,
    q: CallbackQuery,
    storage: Arc<InMemStorage<PromptDialogueState>>,
) -> Result<(), TgError> {
    if let Some(action) = &q.data {
        match action.as_str() {
            // main-menu
            BUY => handle_buy_callback(&bot, &q).await?,
            MAIN_MENU => handle_menu_callback(&bot, &q).await?,
            CLOSE => handle_close_callback(&bot, &q).await?,

            // sub-menus
            _ => match matching_sub_menu(&bot, &q) {
                Some(SubMenuType::SendBuyTx) => match BuyButtons::new(action) {
                    BuyButtons::SendBuyTx => handle_send_tx_callback(&bot, &q).await?,
                    BuyButtons::PrivateTx(_) => handle_private_tx_callback(&bot, &q).await?,
                    BuyButtons::Rebate(_) => handle_rebate_callback(&bot, &q).await?,
                    BuyButtons::Wallet1(_) | BuyButtons::Wallet2(_) | BuyButtons::Wallet3(_) => {
                        handle_wallet_callback(&bot, &q).await?
                    }
                    BuyButtons::BuyToken => {
                        handle_buy_token_callback(
                            &bot,
                            PromptDialogueState::BuyStartAddressPrompt,
                            &q,
                            storage,
                        )
                        .await?
                    }
                    BuyButtons::ReceiveToken => {
                        handle_receive_token_callback(
                            &bot,
                            PromptDialogueState::ReceiveStartAddressPrompt,
                            &q,
                            storage,
                        )
                        .await?
                    }
                    BuyButtons::BuyAmount => {
                        handle_buy_amount_callback(
                            &bot,
                            PromptDialogueState::StartBuyAmountPrompt,
                            &q,
                            storage,
                        )
                        .await?
                    }
                    _ => {}
                },
                Some(SubMenuType::SendSellTx) => {
                    todo!()
                }
                _ => {}
            },
        }
        log::info!("You chose: {}", action);
    }
    Ok(())
}

impl fmt::Display for TgError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Parse(ref err) => write!(f, "Parse error: {}", err),
            Self::TeloxideRequest(ref err) => {
                write!(f, "Telegram request error: {}", err)
            }
            Self::TeloxideInMemStorageError(ref err) => {
                write!(f, "InMemStorage error: {}", err)
            }
            Self::UnmatchedQuery(ref cb_query) => {
                write!(f, "Could not match callback query: {:?}", cb_query)
            }
            Self::NoQueryData(ref cb_query) => {
                write!(f, "Could not get query data: {:?}", cb_query)
            }
            Self::NoQueryMessage(ref cb_query) => {
                write!(f, "Could not get query message: {:?}", cb_query)
            }
            Self::UserNotFound(ref msg) => {
                write!(f, "Could not find user for message: {:?}", msg)
            }
            Self::AnyhowError(ref err) => write!(f, "Anyhow error: {}", err),
        }
    }
}

impl From<teloxide::RequestError> for TgError {
    fn from(err: teloxide::RequestError) -> Self {
        Self::TeloxideRequest(err)
    }
}

impl From<InMemStorageError> for TgError {
    fn from(err: InMemStorageError) -> Self {
        Self::TeloxideInMemStorageError(err)
    }
}

impl From<anyhow::Error> for TgError {
    fn from(err: anyhow::Error) -> Self {
        Self::AnyhowError(err)
    }
}
