use crate::models::ToClaims;
use chrono::Utc;
use hex;
use hmac::{Hmac, Mac};
use jwt::claims::SecondsSinceEpoch;
use jwt::{
    AlgorithmType, Claims, Header, RegisteredClaims, SignWithKey, SigningAlgorithm, Token,
    Verified, VerifyWithKey,
};
use rand::{RngCore, SeedableRng};
use serde::Serialize;
use serde_json::Value;
use sha2::{Digest, Sha512};
use std::collections::BTreeMap;

pub struct Authenticator {
    key: Hmac<Sha512>,
    algorithm: AlgorithmType,
}

impl Authenticator {
    pub fn new(key: &str, algorithm: AlgorithmType) -> Self {
        Authenticator {
            key: Hmac::new_from_slice(key.as_ref()).expect("must work"),
            algorithm,
        }
    }

    pub fn verify_token<'a>(
        &self,
        token: &str,
    ) -> Result<Token<Header, BTreeMap<String, Value>, Verified>, jwt::Error> {
        let token1 = token.verify_with_key(&self.key);
        return token1;
    }
    pub fn generate_token(
        &self,
        user: &crate::models::claims::Claims,
        duration_days: i32,
        issuer: Option<String>,
        audience: Option<String>,
    ) -> String {
        //key.to_base64();
        let mut claims: jwt::Claims = Claims::default();

        let expiration = Utc::now()
            .checked_add_signed(
                chrono::Duration::try_days(duration_days as i64).expect("valid time"),
            )
            .expect("valid timestamp")
            .timestamp() as SecondsSinceEpoch;

        let expiration = Some(expiration);

        claims.registered = RegisteredClaims {
            issuer,
            subject: None,
            audience,
            expiration,
            not_before: None,
            issued_at: None,
            json_web_token_id: None,
        };
        claims.private = user.to_jwt_claims();
        //let d = crate::models::Claims::from_user(user).serialize(FlatMapSerializer);
        //let d = Hmac::new_from_slice("".as_ref()).unwrap();
        let claims = claims;
        //self.key.to_base64();
        //let t = Token::
        let token = Token::new(
            Header {
                algorithm: self.algorithm,
                ..Default::default()
            },
            claims,
        )
        .sign_with_key(&self.key)
        .unwrap();
        token.as_str().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hasher::{check_password, hash};

    #[test]
    fn test_hash() {
        let auth = Authenticator::new("key", AlgorithmType::Hs512);
        let user = crate::models::claims::Claims {
            id: 1,
            email: "test",
            name: "test",
            image: "User",
        };
        let t = auth.generate_token(&user, 13, None, None);
        println!("{}", t);
        println!("{:?}, {:?}", auth.key, auth.algorithm);
    }
}
