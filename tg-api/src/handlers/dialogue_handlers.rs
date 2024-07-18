use crate::bot::TgError;
use crate::consts::{BOT_NAME, BUY_TOKEN, RECEIVE_TOKEN};
use crate::handlers::delete_up_to_messages;
use crate::handlers::find_keyboard_from_message;
use crate::requests::on_chain;
use crate::storages::{TgMessageStorage, GLOBAL_BUY_MENU_STORAGE};
use ethers::types::Address;
use std::str::FromStr;
use teloxide::{
    dispatching::dialogue::{Dialogue, InMemStorage},
    payloads::EditMessageTextSetters,
    requests::Requester,
    types::{InlineKeyboardButtonKind, Message, ParseMode},
    Bot,
};

pub(crate) type BuyAddressPromptDialogue =
    Dialogue<PromptDialogueState, InMemStorage<PromptDialogueState>>;

/// Dialogue state
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub(crate) enum PromptDialogueState {
    #[default]
    /// Represents state when the buy menu buy token button clicked
    BuyStartAddressPrompt,
    /// Represents state when the buy menu buy token address is received
    BuyAddressReceived,
    /// Represents state when the buy menu buy token name is received
    BuyTokenNameReceived,
    /// Represents state when the buy menu receive token button clicked
    ReceiveStartAddressPrompt,
    /// Represents state when the buy menu receive token address is received
    ReceiveAddressReceived,
    /// Represents state when the buy menu receive token name is received
    ReceiveTokenNameReceived,
    /// Represents state when the buy amount button is clicked
    StartBuyAmountPrompt,
    /// Represents state when the buy amount is received
    BuyAmountReceived,
}

pub(crate) async fn buy_address_dialogue_handler(
    bot: Bot,
    dialogue: BuyAddressPromptDialogue,
    msg: Message,
) -> Result<(), TgError> {
    bot.send_message(
        msg.chat.id,
        "Enter the address of the token you want to trade",
    )
    .await?;

    dialogue
        .update(PromptDialogueState::BuyAddressReceived)
        .await?;

    Ok(())
}

pub(crate) async fn buy_address_or_token_handler(
    bot: Bot,
    dialogue: BuyAddressPromptDialogue,
    msg: Message,
) -> Result<(), TgError> {
    let text = match msg.text() {
        Some(t) => t,
        _ => {
            bot.send_message(msg.chat.id, "Send me plain text.").await?;
            return Ok(());
        }
    };

    // Checks if it's valid address
    if text.starts_with("0x") && Address::from_str(text).is_ok() {
        let menu_msg = on_chain::get_on_chain_info().await?;

        if let Some(menu) = GLOBAL_BUY_MENU_STORAGE.get(BOT_NAME.to_string()) {
            let buy_sell_msg = menu.message;
            let buy_sell_msg_id = menu.message_id;
            let keyboard = find_keyboard_from_message(&buy_sell_msg)?;
            let mut new_keyboard = keyboard.clone();

            // Gets the dialogue state
            match dialogue.get().await? {
                Some(PromptDialogueState::BuyAddressReceived) => {
                    let new_button_text = format!("{}", text);
                    if let Some(button) = new_keyboard
                        .inline_keyboard
                        .get_mut(4)
                        .and_then(|row| row.get_mut(0))
                    {
                        button.text = new_button_text.to_string();
                        button.kind = InlineKeyboardButtonKind::CallbackData(BUY_TOKEN.to_string());
                    };
                }
                Some(PromptDialogueState::ReceiveAddressReceived) => {
                    let new_button_text = format!("{}", text);
                    if let Some(button) = new_keyboard
                        .inline_keyboard
                        .get_mut(4)
                        .and_then(|row| row.get_mut(1))
                    {
                        button.text = new_button_text.to_string();
                        button.kind =
                            InlineKeyboardButtonKind::CallbackData(RECEIVE_TOKEN.to_string());
                    };
                }
                _ => {
                    log::warn!("No dialogue found")
                }
            }

            // Edit the message with the new keyboard
            bot.edit_message_text(msg.chat.id, buy_sell_msg_id, menu_msg)
                .parse_mode(ParseMode::MarkdownV2)
                .reply_markup(new_keyboard)
                .await?;
            dialogue.exit().await?;

            let _ = delete_up_to_messages(&bot, msg.chat.id.0, msg.id.0, buy_sell_msg_id.0).await?;
        } else {
            log::warn!("message not found");
        }
    } else {
        bot.send_message(msg.chat.id, "Please enter valid address")
            .await?;
    };

    Ok(())
}

pub(crate) async fn buy_amount_dialogue_handler(
    bot: Bot,
    dialogue: BuyAddressPromptDialogue,
    msg: Message,
) -> Result<(), TgError> {
    let text = match msg.text() {
        Some(t) => t,
        _ => {
            bot.send_message(msg.chat.id, "Send me plain text.").await?;
            return Ok(());
        }
    };
    let is_numeric =
        |value: &str| -> bool { value.parse::<f64>().is_ok() || value.parse::<i64>().is_ok() };

    // Checks if it's is numeric value
    if is_numeric(text) {
        let menu_msg = on_chain::get_on_chain_info().await?;

        if let Some(menu) = GLOBAL_BUY_MENU_STORAGE.get(BOT_NAME.to_string()) {
            let buy_sell_msg = menu.message;
            let buy_sell_msg_id = menu.message_id;
            let keyboard = find_keyboard_from_message(&buy_sell_msg)?;
            let mut new_keyboard = keyboard.clone();

            // Gets the dialogue state
            let new_button_text = format!("{}", text);
            if let Some(button) = new_keyboard
                .inline_keyboard
                .get_mut(5)
                .and_then(|row| row.get_mut(0))
            {
                button.text = new_button_text.to_string();
                button.kind = InlineKeyboardButtonKind::CallbackData(BUY_TOKEN.to_string());
            };
            // Edit the message with the new keyboard
            bot.edit_message_text(msg.chat.id, buy_sell_msg_id, menu_msg)
                .parse_mode(ParseMode::MarkdownV2)
                .reply_markup(new_keyboard)
                .await?;
            dialogue.exit().await?;

            let _ = delete_up_to_messages(&bot, msg.chat.id.0, msg.id.0, buy_sell_msg_id.0).await?;
        } else {
            log::warn!("message not found");
        }
    } else {
        bot.send_message(msg.chat.id, "Please enter numeric value")
            .await?;
    };

    Ok(())
}
