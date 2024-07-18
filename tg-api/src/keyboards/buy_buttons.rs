use crate::consts::{
    BUY_AMOUNT, BUY_TOKEN, CLOSE, ESTIMATED_RECEIVED_AMOUNT, MAIN_MENU, PRIVATE_TX, REBATE,
    RECEIVE_TOKEN, SEND_BUY_TX, SEND_SELL_TX, WALLET_1, WALLET_2, WALLET_3,
};
use crate::keyboards::add_emoji;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

#[derive(Debug, Clone)]
pub(crate) enum BuyButtons<'a> {
    SendBuyTx,
    SendSellTx,
    MainMenu,
    Close,
    PrivateTx(&'a str),
    Rebate(&'a str),
    Wallet1(&'a str),
    Wallet2(&'a str),
    Wallet3(&'a str),
    BuyToken,
    ReceiveToken,
    BuyAmount,
    EstimatedReceivedAmount,
}

impl<'a> BuyButtons<'a> {
    pub(crate) fn new(text: &'a str) -> Self {
        match text {
            t if t == SEND_BUY_TX || t == add_emoji(SEND_BUY_TX).as_str() => Self::SendBuyTx,
            t if t == SEND_SELL_TX || t == add_emoji(SEND_SELL_TX).as_str() => Self::SendSellTx,
            t if t == MAIN_MENU || t == add_emoji(MAIN_MENU).as_str() => Self::MainMenu,
            t if t == CLOSE || t == add_emoji(CLOSE).as_str() => Self::Close,
            t if t == PRIVATE_TX || t == add_emoji(PRIVATE_TX).as_str() => Self::PrivateTx(text),
            t if t == REBATE || t == add_emoji(REBATE).as_str() => Self::Rebate(text),
            t if t == WALLET_1 || t == add_emoji(WALLET_1).as_str() => Self::Wallet1(text),
            t if t == WALLET_2 || t == add_emoji(WALLET_2).as_str() => Self::Wallet2(text),
            t if t == WALLET_3 || t == add_emoji(WALLET_3).as_str() => Self::Wallet3(text),
            BUY_TOKEN => Self::BuyToken,
            RECEIVE_TOKEN => Self::ReceiveToken,
            BUY_AMOUNT => Self::BuyAmount,
            ESTIMATED_RECEIVED_AMOUNT => Self::EstimatedReceivedAmount,
            _ => Self::SendSellTx,
        }
    }

    pub(crate) fn toggle(&self) -> String {
        match self {
            Self::PrivateTx(text) => self.toggle_text(text, PRIVATE_TX),
            Self::Rebate(text) => self.toggle_text(text, REBATE),
            Self::Wallet1(text) => self.toggle_text(text, WALLET_1),
            Self::Wallet2(text) => self.toggle_text(text, WALLET_2),
            Self::Wallet3(text) => self.toggle_text(text, WALLET_3),
            _ => format!("{:?}", self),
        }
    }

    fn toggle_text(&self, current: &str, default: &str) -> String {
        if current == default {
            add_emoji(default)
        } else {
            default.to_string()
        }
    }
}

/// Create the Buy keyboard layout
/// Note: any change to this function will affect the handle_send_tx function() and handle_private_tx_callback()
fn create_buy_keyboard(
    private_tx: bool,
    rebate: bool,
    wallet1: bool,
    wallet2: bool,
    wallet3: bool,
) -> anyhow::Result<InlineKeyboardMarkup> {
    if [wallet1, wallet2, wallet3].iter().filter(|&&x| x).count() != 1 {
        return Err(anyhow::anyhow!("Only one wallet can be selected"));
    };

    let mut keyboard = InlineKeyboardMarkup::default();

    // 1st row
    keyboard = keyboard.append_row(vec![
        // no need to add emoji in the callback value
        InlineKeyboardButton::callback(add_emoji(MAIN_MENU), MAIN_MENU.to_owned()),
        InlineKeyboardButton::callback(add_emoji(CLOSE), CLOSE.to_owned()),
    ]);

    // 2nd row
    keyboard = keyboard.append_row(vec![
        match private_tx {
            true => InlineKeyboardButton::callback(add_emoji(PRIVATE_TX), add_emoji(PRIVATE_TX)),
            false => InlineKeyboardButton::callback(PRIVATE_TX.to_owned(), PRIVATE_TX.to_owned()),
        },
        match rebate {
            true => InlineKeyboardButton::callback(add_emoji(REBATE), add_emoji(REBATE)),
            false => InlineKeyboardButton::callback(REBATE.to_owned(), REBATE.to_owned()),
        },
    ]);

    // 3rd row
    keyboard = keyboard.append_row(vec![InlineKeyboardButton::callback(
        "=Select Wallet=".to_owned(),
        "=Select Wallet=".to_owned(),
    )]);

    // 4th row
    // Default selection to wallet 1
    keyboard = keyboard.append_row(vec![
        match wallet1 {
            true => InlineKeyboardButton::callback(add_emoji(WALLET_1), add_emoji(WALLET_1)),
            false => InlineKeyboardButton::callback(WALLET_1.to_owned(), WALLET_1.to_owned()),
        },
        match wallet2 {
            true => InlineKeyboardButton::callback(add_emoji(WALLET_2), add_emoji(WALLET_2)),
            false => InlineKeyboardButton::callback(WALLET_2.to_owned(), WALLET_2.to_owned()),
        },
        match wallet3 {
            true => InlineKeyboardButton::callback(add_emoji(WALLET_3), add_emoji(WALLET_3)),
            false => InlineKeyboardButton::callback(WALLET_3.to_owned(), WALLET_3.to_owned()),
        },
    ]);

    // 5th row
    keyboard = keyboard.append_row(vec![
        InlineKeyboardButton::callback(BUY_TOKEN.to_owned(), BUY_TOKEN.to_owned()),
        InlineKeyboardButton::callback(RECEIVE_TOKEN.to_owned(), RECEIVE_TOKEN.to_owned()),
    ]);

    // 6th row
    keyboard = keyboard.append_row(vec![InlineKeyboardButton::callback(
        BUY_AMOUNT.to_owned(),
        BUY_AMOUNT.to_owned(),
    )]);

    // 7th row
    keyboard = keyboard.append_row(vec![InlineKeyboardButton::callback(
        ESTIMATED_RECEIVED_AMOUNT.to_owned(),
        ESTIMATED_RECEIVED_AMOUNT.to_owned(),
    )]);

    // 8th row
    // Last one will always be Send Buy Tx
    keyboard = keyboard.append_row(vec![InlineKeyboardButton::callback(
        SEND_BUY_TX.to_owned(),
        SEND_BUY_TX.to_owned(),
    )]);

    Ok(keyboard)
}

pub(crate) fn buy_keyboard(
    private_tx: bool,
    rebate: bool,
    wallet1: bool,
    wallet2: bool,
    wallet3: bool,
) -> anyhow::Result<InlineKeyboardMarkup> {
    match create_buy_keyboard(private_tx, rebate, wallet1, wallet2, wallet3) {
        Ok(keyboard) => Ok(keyboard),
        _ => Err(anyhow::anyhow!("Error creating keyboard")),
    }
}
