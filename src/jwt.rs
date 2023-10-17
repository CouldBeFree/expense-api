use actix_web::dev::Payload;
use actix_web::{Error, FromRequest, HttpRequest};
use actix_web::error::ErrorUnauthorized;
use futures::future::{Ready, ok, err};
use dotenv::dotenv;

use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};

use chrono::Utc;

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtToken {
    pub user_id: String,
    pub exp: usize
}

impl JwtToken {
    pub fn get_key() -> String {
        dotenv().ok();
        let key_str = std::env::var("JWT_SECRET").expect("JWT_SECRET not set");
                            
        return key_str.to_owned();
    }

    pub fn encode(self) -> String {
        let key = EncodingKey::from_secret(JwtToken::get_key().as_ref());
        let token = encode(&Header::default(), &self, &key).unwrap();

        return token;
    }

    pub fn new(user_id: String) -> Self {
        let minutes = std::env::var("EXPIRE_MINUTES")
            .expect("EXPIRE_MINUTES not set")
            .parse::<i64>().expect("Cannot convert expire minutes to i64");

        let expiration = Utc::now()
                        .checked_add_signed(chrono::Duration::minutes(minutes))
                        .expect("valid timestamp")
                        .timestamp();

        return JwtToken { user_id, exp: expiration as usize }
    }

    pub fn from_token(token: String) -> Result<Self, String> {
        let key = DecodingKey::from_secret(JwtToken::get_key().as_ref());
        let token_result = decode::<JwtToken>(&token.as_str(), &key,&Validation::default());

        match token_result {
            Ok(data) => {
                return Ok(data.claims)
            },
            Err(error) => {
                let message = format!("{}", error);
                return Err(message)
            }
        }
    }
}

impl FromRequest for JwtToken {
    type Error = Error;
    type Future = Ready<Result<JwtToken, Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        match req.headers().get("token") {
            Some(data) => {
                let raw_token = data.to_str().unwrap().to_string();
                let token_result = JwtToken::from_token(raw_token);

                match token_result {
                    Ok(token) => {
                        return ok(token)
                    },
                    Err(message) => {
                        if message == "ExpiredSignature".to_owned() {
                            return err(ErrorUnauthorized("token expired"))
                        }
                        return err(ErrorUnauthorized("token can't be decoded"))
                    }
                }
            },
            None => {
                return err(ErrorUnauthorized("token not in header under key 'token'"))
            }
        }
    }
}
