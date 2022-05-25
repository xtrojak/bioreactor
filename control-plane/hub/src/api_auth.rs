use crate::{ErrorMessage, HubConfig};
use chrono::{DateTime, Utc};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::response::status::Forbidden;
use rocket::serde::json::Json;
use rocket::{Build, Request, Rocket, State};
use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_256};
use std::time::{Duration, SystemTime};

const AUTH_HTTP_HEADER: &str = "Authorization";
const DURATION_MONTH: Duration = Duration::from_secs(30 * 24 * 60 * 60);

/// Register API endpoints responsible for authentication.
pub fn register(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket.mount("/", routes![login, renew, login_cors])
}

/// Cryptographic proof of authorization that is passed (`base64` encoded) in a HTTP header.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ApiToken {
    hash: String,
    expires: SystemTime,
}

/// Data object used during login.
#[derive(Debug, Clone, Deserialize)]
struct Credentials {
    password: String,
}

/// Data object used to return a fresh `ApiToken`.
#[derive(Debug, Clone, Serialize)]
struct FreshToken {
    token: String,
    expires: DateTime<Utc>,
}

/// Login endpoint that converts a valid password to a valid access token.
#[post("/login", format = "json", data = "<credentials>")]
fn login(
    credentials: Json<Credentials>,
    config: &State<HubConfig>,
) -> Result<Json<FreshToken>, Forbidden<Json<ErrorMessage>>> {
    let credentials = credentials.into_inner();
    if config.user_password.contains(&credentials.password) {
        let token = ApiToken::new(config.server_password.as_str(), DURATION_MONTH);
        Ok(Json::from(FreshToken {
            token: token.to_base64(),
            expires: token.expires.into(),
        }))
    } else {
        let message: ErrorMessage = "Invalid password.".into();
        Err(Forbidden(Some(Json::from(message))))
    }
}

#[options("/login")]
fn login_cors() {
    // TODO: We should resolve this in a more universal way.
}

/// Renew a still valid token with a new one with longer expiration date.
///
/// Note that the `_token` argument is only used as a request guard to ensure authentication
/// and the token itself is not necessary to generate the new token.
#[get("/renew", format = "json")]
fn renew(config: &State<HubConfig>, _token: ApiToken) -> Json<FreshToken> {
    let token = ApiToken::new(config.server_password.as_str(), DURATION_MONTH);
    Json::from(FreshToken {
        token: token.to_base64(),
        expires: token.expires.into(),
    })
}

impl ApiToken {
    /// Create a new `ApiToken` using a `secret` key. The token is valid for the given `duration`.
    ///
    /// Under normal circumstances, this function should not panic. However, it may panic
    /// if invoked before the the start of the unix epoch or when the unix time overflows 64 bits.
    pub fn new(secret: &str, duration: Duration) -> ApiToken {
        let expires = SystemTime::now() + duration;
        let timestamp = Self::timestamp(expires);
        let hash = Self::hash_secret(timestamp, secret);
        ApiToken { expires, hash }
    }

    /// Try to parse the `ApiToken` from a `base64` encoded string. Returns `None` if the token
    /// is malformed.
    pub fn try_from_base64(token: &str) -> Option<ApiToken> {
        if let Ok(buffer) = base64::decode(token) {
            if let Ok(raw_token) = String::from_utf8(buffer) {
                let mut parts = raw_token.split(';');
                let expires = parts.next()?.parse::<u64>().ok()?;
                let expires = Self::time(expires);
                let hash = parts.next()?.to_string();

                // Check that there are only two parts in the token.
                if parts.next().is_some() {
                    return None;
                }

                return Some(ApiToken { expires, hash });
            }
        }

        None
    }

    /// Convert this `ApiToken` to a base64 encoded string.
    pub fn to_base64(&self) -> String {
        base64::encode(format!("{};{}", Self::timestamp(self.expires), self.hash))
    }

    /// Returns `true` if the expiration time of this `ApiToken` was before the current
    /// system time.
    pub fn is_expired(&self) -> bool {
        self.expires < SystemTime::now()
    }

    /// Returns `true` if the provided `secret` is the same as the one used for generating
    /// this `ApiToken`.
    pub fn check_secret(&self, secret: &str) -> bool {
        let expected = Self::hash_secret(Self::timestamp(self.expires), secret);
        self.hash == expected
    }

    /// Utility method to safely convert time to timestamp.
    fn timestamp(time: SystemTime) -> u64 {
        let stamp = time
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        u64::try_from(stamp).unwrap()
    }

    /// Utility method to safely convert timestamp to time.
    fn time(timestamp: u64) -> SystemTime {
        SystemTime::UNIX_EPOCH + Duration::from_millis(timestamp)
    }

    /// Utility method to compute token hash from secret data.
    fn hash_secret(timestamp: u64, secret: &str) -> String {
        let raw_secret = format!("{};{}", timestamp, secret);
        let mut hasher = Sha3_256::new();
        hasher.update(&raw_secret);
        let hash = hasher.finalize();
        base64::encode(&hash)
    }
}

/// Allows to use `ApiToken` as a request guard, effectively ensuring that every request
/// that takes `ApiToken` as a parameter is authenticated using the token.
#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiToken {
    type Error = &'static str;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Read server config.
        let config = if let Some(it) = request.rocket().state::<HubConfig>() {
            it
        } else {
            return Outcome::Failure((Status::InternalServerError, "Server configuration error."));
        };

        // Read token from header.
        let token = if let Some(header) = request.headers().get_one(AUTH_HTTP_HEADER) {
            if let Some(it) = ApiToken::try_from_base64(header) {
                it
            } else {
                return Outcome::Failure((
                    Status::BadRequest,
                    "Malformed token in `Authorization` header.",
                ));
            }
        } else {
            return Outcome::Failure((Status::Unauthorized, "Missing `Authorization` header."));
        };

        // Check token validity.
        if token.is_expired() {
            return Outcome::Failure((Status::Unauthorized, "Provided token is expired."));
        }

        if !token.check_secret(config.server_password.as_str()) {
            return Outcome::Failure((Status::Unauthorized, "Provided token is invalid."));
        }

        Outcome::Success(token)
    }
}
