use api::{
    account::db::AccountId,
    character::{
        db::{CharacterDB, CharacterId, DBError, DBResult},
        Character,
    },
};
use async_std::sync::RwLock;
use std::collections::{hash_map::Entry, HashMap};
use tracing::{debug, info};

pub struct InMemoryCharacterDB {
    verbose: bool,
    characters: RwLock<HashMap<CharacterId, Character>>,
    accounts: RwLock<HashMap<AccountId, Vec<CharacterId>>>,
}

impl InMemoryCharacterDB {
    pub async fn new(verbose: bool) -> DBResult<Self> {
        let mut s = Self {
            verbose,
            characters: RwLock::new(HashMap::new()),
            accounts: RwLock::new(HashMap::new()),
        };
        s.init().await?;
        Ok(s)
    }
}

#[async_trait::async_trait]
impl CharacterDB for InMemoryCharacterDB {
    async fn init(&mut self) -> DBResult<()> {
        if self.verbose {
            info!("Initializing InMemory character DB");
        }
        // self.accounts.write().await.insert(2000042, vec![2_000_000]);
        // self.characters
        //     .write()
        //     .await
        //     .insert(2_000_000, Character::new(2_000_000, 2000042));
        Ok(())
    }

    async fn create(&self, account_id: AccountId) -> DBResult<CharacterId> {
        if self.verbose {
            debug!(%account_id, "Creating a new character");
        }
        let mut char_id = fastrand::u32(2_000_000..);
        loop {
            let mut chars = self.characters.write().await;
            match chars.entry(char_id) {
                Entry::Occupied(_) => {}
                Entry::Vacant(e) => {
                    e.insert(Character::new(char_id, account_id));
                    break;
                }
            }
            char_id = fastrand::u32(2_000_000..);
        }

        match self.accounts.write().await.entry(account_id) {
            Entry::Occupied(mut chars) => {
                chars.get_mut().push(char_id);
            }
            Entry::Vacant(e) => {
                e.insert(vec![char_id]);
            }
        }
        Ok(char_id)
    }

    async fn update(&self, character: &Character) -> DBResult<()> {
        self.characters
            .write()
            .await
            .insert(character.id, character.clone());
        Ok(())
    }

    async fn delete(&self, _id: CharacterId) -> DBResult<()> {
        todo!("char db deletion")
    }

    async fn get_by_account_id(
        &self,
        id: api::account::db::AccountId,
    ) -> DBResult<Vec<api::character::Character>> {
        let char_ids = self
            .accounts
            .read()
            .await
            .get(&id)
            .cloned()
            .unwrap_or_default();

        let char_db = self.characters.read().await;
        let chars = char_ids
            .into_iter()
            .filter_map(|id| char_db.get(&id))
            .cloned()
            .collect();
        Ok(chars)
    }

    async fn get_by_id(&self, id: CharacterId) -> DBResult<api::character::Character> {
        self.characters
            .read()
            .await
            .get(&id)
            .cloned()
            .ok_or_else(|| DBError::NoSuchCharacter(id))
    }
}
