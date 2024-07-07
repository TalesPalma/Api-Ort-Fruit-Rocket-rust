use chrono::{TimeDelta, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::Serialize;

use crate::models::administrator::Administrator;

pub fn login(email: String, password: String) -> Result<Administrator, String> {
    if email == "admin" && password == "admin" {
        Ok(Administrator {
            id: 1,
            name: "admin".to_string(),
            email: "admin".to_string(),
            password: "admin".to_string(),
            token: gerar_token_jwt(),
        })
    } else {
        Err("Email ou senha invalidos".to_string())
    }
}

fn gerar_token_jwt() -> String {
    let expiration = Utc::now()
        .checked_add_signed(TimeDelta::hours(24))
        .expect("Invalid timestamp")
        .timestamp();

    #[derive(Serialize)]
    struct Chaims {
        sub: String,
        exp: String,
    }

    let chaims = Chaims {
        sub: "1231".to_string(),
        exp: expiration.to_string(),
    };

    encode(
        &Header::default(),
        &chaims,
        &EncodingKey::from_secret("Your scret key".as_ref()),
    )
    .unwrap()
    .to_string()
}
