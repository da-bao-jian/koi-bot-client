use hashbrown::HashMap;
use lazy_static::lazy_static;
use parking_lot::RwLock;
use std::sync::Arc;
use teloxide::types::{ChatId, Message, MessageId};

lazy_static! {
    /// Used to locate the main menu location
    pub(crate) static ref GLOBAL_MAIN_MENU_STORAGE: MainMenuStorage = MainMenuStorage::new();
}

lazy_static! {
    /// Used to locate the buy menu location
    pub(crate) static ref GLOBAL_BUY_MENU_STORAGE: BuyMenuStorage = TgMessageStorage::new();
}

lazy_static! {
    /// Used to locate the sell menu location
    pub(crate) static ref GLOBAL_SELL_MENU_STORAGE: SellMenuStorage = TgMessageStorage::new();
}

pub(crate) trait TgMessageStorage {
    fn new() -> Self;
    fn insert(&self, user_name: String, message: TgMessage);
    fn get(&self, user_name: String) -> Option<TgMessage>;
    fn remove(&self, user_name: String) -> Option<TgMessage>;
    fn delete_all(&self);
}

#[derive(Debug, Clone)]
pub(crate) struct TgMessage {
    pub(crate) chat_id: ChatId,
    pub(crate) message_id: MessageId,
    pub(crate) message: Arc<Message>,
}

#[derive(Debug, Default)]
pub(crate) struct BuyMenuStorage {
    storage: Arc<RwLock<HashMap<String, TgMessage>>>,
}

impl TgMessageStorage for BuyMenuStorage {
    fn new() -> Self {
        BuyMenuStorage {
            storage: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    fn insert(&self, user_name: String, message: TgMessage) {
        let mut storage = self.storage.write();
        storage.insert(user_name, message);
    }

    fn get(&self, user_name: String) -> Option<TgMessage> {
        let storage = self.storage.read();
        storage.get(&user_name).cloned()
    }

    fn remove(&self, user_name: String) -> Option<TgMessage> {
        let mut storage = self.storage.write();
        storage.remove(&user_name)
    }

    fn delete_all(&self) {
        let mut storage = self.storage.write();
        storage.clear();
    }
}

#[derive(Debug, Default)]
pub(crate) struct SellMenuStorage {
    storage: Arc<RwLock<HashMap<String, TgMessage>>>,
}

impl TgMessageStorage for SellMenuStorage {
    fn new() -> Self {
        SellMenuStorage {
            storage: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    fn insert(&self, user_name: String, message: TgMessage) {
        let mut storage = self.storage.write();
        storage.insert(user_name, message);
    }

    fn get(&self, user_name: String) -> Option<TgMessage> {
        let storage = self.storage.read();
        storage.get(&user_name).cloned()
    }

    fn remove(&self, user_name: String) -> Option<TgMessage> {
        let mut storage = self.storage.write();
        storage.remove(&user_name)
    }

    fn delete_all(&self) {
        let mut storage = self.storage.write();
        storage.clear();
    }
}

#[derive(Debug, Default)]
pub(crate) struct MainMenuStorage {
    storage: Arc<RwLock<HashMap<String, TgMessage>>>,
}

impl TgMessageStorage for MainMenuStorage {
    fn new() -> Self {
        MainMenuStorage {
            storage: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    fn insert(&self, user_name: String, message: TgMessage) {
        let mut storage = self.storage.write();
        storage.insert(user_name, message);
    }

    fn get(&self, user_name: String) -> Option<TgMessage> {
        let storage = self.storage.read();
        storage.get(&user_name).cloned()
    }

    fn remove(&self, user_name: String) -> Option<TgMessage> {
        let mut storage = self.storage.write();
        storage.remove(&user_name)
    }

    fn delete_all(&self) {
        let mut storage = self.storage.write();
        storage.clear();
    }
}
