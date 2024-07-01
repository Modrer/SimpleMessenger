use jwt::claims::SecondsSinceEpoch;
use rocket::{form, request, Request};
use rocket::data::ToByteUnit;
use rocket::form::{DataField, FromFormField, ValueField};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::serde::{Deserialize, Serialize};

const BEARER: &str = "Bearer ";
#[derive(Debug)]

pub struct Token(pub(crate) String);

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Claims {
    #[serde(rename = "iss", skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    #[serde(rename = "aud", skip_serializing_if = "Option::is_none")]
    pub audience: Option<String>,
    #[serde(rename = "exp", skip_serializing_if = "Option::is_none")]
    pub expiration: Option<SecondsSinceEpoch>,

    pub id: i32,
    pub name: String,
    pub email: String,
    pub image: String,
}

#[derive(Debug)]
pub enum ApiTokenError {
    Missing,
    Invalid,
}

impl Claims {
    fn from_token(token: &str) -> Result<Self, ()> {
        let binding = authentification::registrator::verify_token(&token);

        if let Err(_) = binding {
            return Err(())?;
        }
        let binding = binding.unwrap();

        let result = binding.claims();

        let result = serde_json::to_string(result);

        if let Err(_) = result {
            return Err(())?;
        }

        let result: serde_json::Result<Claims> = serde_json::from_str(&*result.unwrap());

        if let Err(_) = result {
            return Err(())?;
        }

        Ok(result.unwrap())
    }
}

#[rocket::async_trait]
impl<'r> FromFormField<'r> for Claims {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        let token = field.value;
        let claims = Self::from_token(&token);

        if let Err(_) = claims {
            return Err(form::Error::validation("does not work"))?;
        }

        Ok(claims.unwrap())
    }

    async fn from_data(field: DataField<'r, '_>) -> form::Result<'r, Self> {
        let limit = field
            .request
            .limits()
            .get("json")
            .unwrap_or(256.kilobytes());

        // Read the capped data stream, returning a limit error as needed.
        let token = field.data.open(limit).into_string().await?;
        if !token.is_complete() {
            Err((None, Some(limit)))?;
        }
        let token = token.value;

        let claims = Self::from_token(&token);

        if let Err(_) = claims {
            return Err((None, Some(limit)))?;
        }

        Ok(claims.unwrap())
    }
}
#[rocket::async_trait]
impl<'r> FromRequest<'r> for Claims {
    type Error = ApiTokenError;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let token = request.headers().get_one("Authorization");
        if let None = token {
            return Outcome::Error((Status::Unauthorized, ApiTokenError::Missing));
        }

        let token = token.unwrap().trim_start_matches(BEARER);

        let result = Self::from_token(token);

        if let Err(_) = result {
            return Outcome::Error((Status::Unauthorized, ApiTokenError::Invalid));
        }

        Outcome::Success(result.unwrap())
    }
}
