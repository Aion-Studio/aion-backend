use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: String,
    aud: String,
    role: String,
    pub email: String,
    email_confirmed_at: Option<String>,
    phone: Option<String>,
    confirmation_sent_at: Option<String>,
    confirmed_at: Option<String>,
    recovery_sent_at: Option<String>,
    last_sign_in_at: Option<String>,
    pub app_metadata: AppMetadata,
    pub user_metadata: UserMetadata,
    identities: Vec<Identity>,
    created_at: Option<String>,
    updated_at: Option<String>,
    is_anonymous: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AppMetadata {
    pub provider: String,
    pub providers: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserMetadata {
    pub email: String,
    pub email_verified: bool,
    pub phone_verified: bool,
    pub sub: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Identity {
    pub identity_id: String,
    pub id: String,
    pub user_id: String,
    pub identity_data: IdentityData,
    pub provider: String,
    pub last_sign_in_at: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct IdentityData {
    email: String,
    email_verified: bool,
    phone_verified: bool,
    sub: String,
}
