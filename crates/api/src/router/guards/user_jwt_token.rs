use app::user::auth::UserJWTTokenClaims;
use auth::jwt_token::jwt::verify_jwt_token;
use envs::SALT_SECRET;
use rocket::{http::Status, request::FromRequest};

pub struct JWTTokenGuard {
    claims: UserJWTTokenClaims,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for JWTTokenGuard {
    type Error = ();

    async fn from_request(request: &'r rocket::Request<'_>) -> rocket::request::Outcome<Self, Self::Error> {
        // get the token from the request header
        let token = request.headers().get_one("Authorization");

        match token {
            Some(token) => {
                // verify the token1
                let token_data = verify_jwt_token::<UserJWTTokenClaims>(&SALT_SECRET, token);

                match token_data {
                    Ok(token_data) => {
                        let roles = match token_data.claims.get_roles() {
                            Ok(_roles) => _roles,
                            Err(_err) => return rocket::outcome::Outcome::Error((Status::Unauthorized, ())),
                        };

                        if token_data.claims.is_expired() {
                            return rocket::outcome::Outcome::Error((Status::Unauthorized, ()));
                        }
                        if roles.is_empty() {
                            return rocket::outcome::Outcome::Error((Status::Unauthorized, ()));
                        }
                        return rocket::outcome::Outcome::Success(JWTTokenGuard { claims: token_data.claims });
                    }
                    Err(_) => rocket::outcome::Outcome::Error((Status::Unauthorized, ())),
                }
            }
            None => rocket::outcome::Outcome::Error((Status::Unauthorized, ())),
        }
    }
}
