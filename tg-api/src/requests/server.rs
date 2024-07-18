use teloxide::types::InlineKeyboardMarkup;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub(crate) struct SendBuyTxRequest {
    pub(crate) wallet: String,
    pub(crate) private_tx: bool,
    pub(crate) rebate: bool,
    pub(crate) buy: String,
    pub(crate) receive: String,
    pub(crate) buy_amount: f64,
}

#[allow(dead_code)]
impl SendBuyTxRequest {
    ///  function called in handle_send_tx() to extract the [InlineKeyboardButton](cteloxide::types::InlineKeyboardButton) texts
    /// Note: any change to the buy button layout from the [keyboard.rs](crate::keyboards) will affect this function
    pub(crate) fn new(keyboard: &InlineKeyboardMarkup) -> anyhow::Result<Self> {
        // find the wallet that has emoji
        let selected_wallet = match keyboard.inline_keyboard.get(3).and_then(|row| {
            row.iter()
                .find(|&button| {
                    !(button.text == "Wallet 1"
                        || button.text == "Wallet 2"
                        || button.text == "Wallet 3")
                })
                .map(|button| button.text.clone())
        }) {
            Some(wallet) => wallet,
            None => return Err(anyhow::anyhow!("No wallet found")),
        };

        // find it user want private transaction or not
        // if has emoji, then user wants private tx, otherwise no
        let private_tx = match keyboard.inline_keyboard[1][0].text.as_str() {
            "Private Tx" => false,
            _ => true,
        };

        // find if user wants rebate or not
        // if has emoji, then user wants rebate, otherwise no
        let rebate = match keyboard.inline_keyboard[1][1].text.as_str() {
            "Rebate" => false,
            _ => true,
        };

        let buy_token_address = match keyboard.inline_keyboard[4][0]
            .text
            .as_str()
            .split(": ")
            .collect::<Vec<&str>>()
            .get(1)
            .cloned()
        {
            Some(token_address) => token_address,
            None => return Err(anyhow::anyhow!("No token address found")),
        };

        let receive_token_addesss = match keyboard.inline_keyboard[4][1]
            .text
            .as_str()
            .split(": ")
            .collect::<Vec<&str>>()
            .get(1)
            .cloned()
        {
            Some(token_address) => token_address,
            None => return Err(anyhow::anyhow!("No token address found")),
        };

        let buy_amount: f64 = match keyboard.inline_keyboard[5][0]
            .text
            .as_str()
            .split(": ")
            .collect::<Vec<&str>>()
            .get(1)
            .cloned()
        {
            Some(amount) => amount.parse().expect("Unable to parse amount"),
            None => return Err(anyhow::anyhow!("No amount found")),
        };

        Ok(Self {
            wallet: selected_wallet,
            private_tx,
            rebate,
            buy: buy_token_address.to_string(),
            receive: receive_token_addesss.to_string(),
            buy_amount,
        })
    }
}
