use std::env;

use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use tracing::{error, info};

use crate::endpoints::auth::AuthError;

use super::user::User;

#[derive(Debug, Serialize, Deserialize)]
pub enum AuthPayload {
    Token(String),
    Wallet(String),
}

impl AuthPayload {
    pub async fn get_user(&self) -> Result<User, AuthError> {
        match self {
            AuthPayload::Token(supabase_auth_token) => {
                let req_client = reqwest::Client::new();
                let url = format!(
                    "https://{}.supabase.co/auth/v1/user",
                    env::var("SUPABASE_PROJECT_ID").expect("SUPABASE_PROJECT_ID must be set")
                );
                let res = req_client
                    .get(url)
                    .header("Authorization", format!("Bearer {}", supabase_auth_token))
                    .header("apikey", env::var("SUPABASE_API_KEY").unwrap())
                    .send()
                    .await;
                match res {
                    Ok(res) => {
                        info!("[Supabase user check response]: {:?}", res);
                        if res.status() == StatusCode::OK {
                            let user = res.json::<User>().await;
                            if let Ok(user) = user {
                                info!("[Supabase user check]: {:?}", user);
                                return Ok(user);
                            }
                        } else {
                            let msg = res.text().await.unwrap();
                            error!(msg);
                        }
                        return Err(AuthError::InvalidCredentials(anyhow::anyhow!(
                            "Invalid credentials."
                        )));
                    }
                    Err(e) => {
                        error!("[Supabase user check error]: {:?}", e);
                        return Err(AuthError::InvalidCredentials(anyhow::anyhow!(
                            "Invalid credentials."
                        )));
                    }
                };
            }
            AuthPayload::Wallet(user) => Err(AuthError::UnexpectedError(anyhow::anyhow!(
                "Wallet authentication is not implemented."
            ))),
        }
    }
}
