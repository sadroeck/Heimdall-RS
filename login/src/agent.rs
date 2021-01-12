use api::{
    account::db::{AccountDB, AccountId},
    account::mmo_account::{AccountState, MmoAccount, Password},
    character::CharacterServer,
    login::{LoginCredentials, LoginFailed},
};
use dashmap::DashMap;
use std::{
    marker::PhantomData,
    sync::Arc,
    time::{Duration, SystemTime},
};
use tracing::{debug, error, warn};

pub struct LoginAgent<A, C>
where
    A: AccountDB + Send + Sync + 'static,
    C: CharacterServer + Send + Sync + 'static,
{
    account_db: Arc<A>,
    active_users: Arc<DashMap<AccountId, SystemTime>>,
    _phantom: PhantomData<C>,
}

impl<A, C> LoginAgent<A, C>
where
    A: AccountDB + Send + Sync + 'static,
    C: CharacterServer + Send + Sync + 'static,
{
    pub fn new(account_db: Arc<A>) -> Self {
        Self {
            account_db,
            active_users: Arc::new(DashMap::new()),
            _phantom: PhantomData::default(),
        }
    }

    pub async fn authenticate(
        &self,
        credentials: LoginCredentials,
    ) -> Result<MmoAccount, LoginFailed> {
        // Retrieve account
        let mut account = match credentials {
            LoginCredentials::OTP { .. } => {
                todo!("Handle OTPs");
            }
            LoginCredentials::Hashed {
                username, password, ..
            } => {
                debug!(%username, "Logging in");
                match self.account_db.get_account_by_user(&username).await {
                    Ok(account) => {
                        if let Password::MD5Hashed(hashed) = account.password {
                            if password == hashed {
                                Ok(account)
                            } else {
                                warn!(%username, "Invalid password");
                                Err(LoginFailed::IncorrectPassword)
                            }
                        } else {
                            warn!("Invalid password type");
                            Err(LoginFailed::IncorrectPassword)
                        }
                    }
                    Err(err) => {
                        error!(%err);
                        Err(LoginFailed::RejectedFromServer)
                    }
                }
            }
            LoginCredentials::ClearText {
                username, password, ..
            } => {
                debug!(%username, "Logging in");
                match self.account_db.get_account_by_user(&username).await {
                    Ok(account) => {
                        if let Password::Cleartext(cleartext) = &account.password {
                            if password == *cleartext {
                                Ok(account)
                            } else {
                                warn!(%username, "Invalid password");
                                Err(LoginFailed::IncorrectPassword)
                            }
                        } else {
                            warn!("Invalid password type");
                            Err(LoginFailed::IncorrectPassword)
                        }
                    }
                    Err(err) => {
                        error!(%err);
                        Err(LoginFailed::UnregisteredId(username.clone()))
                    }
                }
            }
        }?;

        // Check account state
        match account.state {
            AccountState::Normal => Ok(()),
            AccountState::Banned(until) => {
                if SystemTime::now() > until {
                    account.state = AccountState::Normal;
                    Ok(())
                } else {
                    Err(LoginFailed::BannedUntil(until))
                }
            }
            AccountState::ExpireOn(expiration_time) => {
                if SystemTime::now() >= expiration_time {
                    Err(LoginFailed::IdIsExpired)
                } else {
                    Ok(())
                }
            }
        }?;

        // TODO: Check client hash

        // Update account
        account.login_count += 1;
        account.lastlogin = SystemTime::now();
        self.account_db
            .save_account(&account)
            .await
            .map_err(|err| {
                error!(%err, "Could not save account");
                LoginFailed::RejectedFromServer
            })?;
        Ok(account)
    }

    pub fn create_session(&self, account_id: AccountId) {
        self.active_users.insert(
            account_id,
            SystemTime::now()
                .checked_add(Duration::from_secs(900))
                .unwrap(),
        );
    }
}
