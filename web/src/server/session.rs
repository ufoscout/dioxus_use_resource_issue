use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

const USER_SESSION_DATA_KEY: &str = "user_session_data_key";

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct UserSessionData {
    pub username: String,
}

impl UserSessionData {
    pub fn new(username: String) -> Self {
        Self { username }
    }

    /// Fetchs the user session data from the session data
    pub async fn fetch() -> Result<Option<Self>, ServerFnError> {
        let session = extract_session().await?;
        let data = session.get(USER_SESSION_DATA_KEY).await?;
        Ok(data)
    }

    /// Sets the user session data in the session data
    pub async fn set(&self) -> Result<(), ServerFnError> {
        let session = extract_session().await?;
        session.insert(USER_SESSION_DATA_KEY, self).await?;
        Ok(())
    }

    /// Deletes the user session data from the session data
    pub async fn delete(self) -> Result<Option<Self>, ServerFnError> {
        let session = extract_session().await?;
        let data = session.remove(USER_SESSION_DATA_KEY).await?;
        Ok(data)
    }
    
}

#[inline]
pub async fn extract_session() -> Result<tower_sessions::Session, ServerFnError> {
    extract::<tower_sessions::Session, _>()
        .await
        .map_err(|_| ServerFnError::new("SessionLayer was not found"))
}