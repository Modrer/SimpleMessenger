use crate::models::ToClaims;
use authentification::hasher::hash_password;
use chrono::Utc;
use db_manager::user_mapper;
use dotenv::dotenv;
use hex;
use hmac::Mac;
use jwt::claims::SecondsSinceEpoch;
use jwt::{Claims, Header, RegisteredClaims, SignWithKey, SigningAlgorithm, Token};
use rand::{RngCore, SeedableRng};
use serde::Serialize;
use serde_json::Value;
use sha2::Digest;
use std::collections::BTreeMap;

mod authentication;
mod hasher;
mod models;
mod registrator;

const SALT_LENGTH: usize = 16;

#[tokio::main]
async fn main() {
    match dotenv() {
        Ok(a) => println!("Ok : {:?}", a),
        Err(e) => {
            println!("Err :{:?}", e)
        }
    }
    let var = registrator::register("test123", "test123@testmail.com", "string1").await;
    let hashed = hash_password("string1");

    let var = user_mapper::add_user(
        "test123",
        "test123@testmail.com",
        &hashed.hashed_password,
        &hashed.salt,
    )
    .await;

    match var {
        Ok(i) => println!("{}", i),
        Err(i) => println!("{}", i),
    }

    //println!("{}", var);
    // let mut bytes : [u8; 48] = [0;  48];
    // StdRng::from_entropy().fill_bytes(&mut bytes);
    // println!("{}", SaltString::encode_b64(&bytes).unwrap().as_str().chars().count());
    // println!("{}",
    //          "5DmNLBfFvSe0ud2jEhp5V1Gj46ajLC6xFHN7KINXjHr2PdZsTWwZkVIdOoeMy9R/ldQvrdtcrxCPovXTbjDRdw=="
    //              .chars().count());
    // let mut bytes : [u8; SALT_LENGTH] = [0;  SALT_LENGTH];
    // StdRng::from_entropy().fill_bytes(&mut bytes);
    // //let str = SaltString::encode_b64(&bytes).unwrap().as_str();
    // let binding = SaltString::encode_b64(&bytes).unwrap();
    // let str = binding.as_str();
    // let mut hasher = Sha256::new();
    //
    // let hash = hasher::hash("string",
    //                         "kLg08Phky/Z7epAq7yfxWPzr8Ti8uODngMEn+Z4ENez5VCX7P5BfgMQsMrMzY3NAEB5SVyY5QMGtfcyUQqAnUA=="
    // );
    //
    // println!("{:?}", check_password("string",
    //                                 "kLg08Phky/Z7epAq7yfxWPzr8Ti8uODngMEn+Z4ENez5VCX7P5BfgMQsMrMzY3NAEB5SVyY5QMGtfcyUQqAnUA==",
    //                                 "140c58c5c2c86150c9926495482597579b40813131346d7621c432a412bd9cae"
    // ));
    // let d = data_models::User{
    //     user_id: 0,
    //     name: "test".to_string(),
    //     email: "test".to_string(),
    //     password_hash: "".to_string(),
    //     salt: "".to_string(),
    //     role: "User".to_string(),
    // };
    //
    //
    // let auth = Authenticator::new("key", AlgorithmType::Hs384);
    // let t = auth.generate_token( d,12);
    // println!("{}", t)
    //let key: Hmac<Sha384> = Hmac::new_from_slice(b"some-secret").unwrap();
    // let header = Header {
    //     algorithm: AlgorithmType::Hs384,
    //     ..Default::default()
    // };
    let mut claims: Claims = Claims::new(Default::default());

    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::try_days(12i64).expect("valid time"))
        .expect("valid timestamp")
        .timestamp() as SecondsSinceEpoch;

    let expiration = Some(expiration);

    claims.registered = RegisteredClaims {
        issuer: None,
        subject: None,
        audience: None,
        expiration,
        not_before: None,
        issued_at: None,
        json_web_token_id: None,
    };
    let user = data_models::User {
        user_id: 0,
        name: "test".to_string(),
        email: "test".to_string(),
        password_hash: "".to_string(),
        salt: "".to_string(),
        image: "User".to_string(),
    };

    claims.private = user.to_claims().to_jwt_claims();
    let token = registrator::generate_token(user.user_id, &user.name, &user.email, &user.image);
    //Token::new(header, claims).sign_with_key(&key).unwrap();
    println!("{}", token);
    let token: Token<Header, BTreeMap<String, Value>, _> =
    registrator::verify_token(
        "eyJhbGciOiJIUzUxMiJ9.eyJpc3MiOiIxMjcuMC4wLjE6ODAwMCIsImF1ZCI6IjEyNy4wLjAuMTo4MDAwIiwiZXhwIjoxNzEyMjM1NDY4LCJlbWFpbCI6InRlc3QiLCJpZCI6MCwibmFtZSI6InRlc3QiLCJyb2xlIjoiVXNlciJ9.l-HVT-vXlbMQN8LrEHKBsQiQwBU0Ikdd20jasZov0W1NV1TEhnjTL_z1zS5NK_B3Tpz-5_TSmxj_YOuZWM7E7w"
    ).unwrap();

    let d = token.claims();

    println!("{:?}", d);
    //assert_eq!(token.as_str(), "eyJhbGciOiJIUzM4NCJ9.eyJzdWIiOiJzb21lb25lIn0.WM_WnPUkHK6zm6Wz7zk1kmIxz990Te7nlDjQ3vzcye29szZ-Sj47rLNSTJNzpQd_");
}
