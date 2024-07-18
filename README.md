# Koi bot  - A Telegram Crypto Trading Bot Sample Implementation 🎏

[Screencast from 2024-07-18 23-08-42.webm](https://github.com/user-attachments/assets/c77dbf51-d13b-4766-b200-1a5982d79a22)

## 🚧 WARNING 🚧  

This project is purely experimental. 

This code was not fully tested and will not be actively maintained for production purposes. 

Use at your own risk.

## Background

A sample Telegram crypto trading bot implementation using the [`teloxide`](https://github.com/teloxide/teloxide) framework. 

This bot provides an interface to interact with Telegram's API, enabling users to buy/sell and set limit buy/sell orders.

Please note that the code is only partially complete and does not include backend logic for sending real transactions. To enable real transactions, you'll need to modify the [hard coded callback function](https://github.com/da-bao-jian/koi-bot-client/blob/d882adf19c0d4328f6a7cd98a8d1a457a01b893d/tg-api/src/handlers/callback_handlers.rs#L211) to send actual transactions



## Requirements

1. Create a new bot using @Botfather to get a token 
2. Init the TELOXIDE_TOKEN env variable
```shell
$ export TELOXIDE_TOKEN=<Your token here>
```
3. Set the environment variable `ETH_RPC_URL` in `.env` file

## Running the bot
To see bot in action, 
```shell
cargo run --bin koi-bot
```

