use serde::{Deserialize, Serialize};
use jsonwebtoken::{DecodingKey, EncodingKey};
//use uuid::Uuid;

use sqlx::FromRow;

#[derive(Deserialize, sqlx::FromRow,Serialize)]
pub struct User {
    pub id:String,
    pub name:String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct Product {
    pub id:String,
    pub name:String,
    pub description: String,
    pub quantity: String,
    pub price:String,
}

#[derive(Deserialize, Serialize)]
pub struct Claims {
    pub email: String,
    pub exp: u64,
}

pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}
