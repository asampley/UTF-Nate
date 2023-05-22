use axum::extract::Query;
use axum::response::IntoResponse;
use axum_extra::extract::cookie::Cookie;
use axum_extra::extract::CookieJar;

use crate::commands::token::Encrypted;

pub async fn token(Query(encrypted): Query<Encrypted>, jar: CookieJar) -> impl IntoResponse {
	jar.add(Cookie::new(
		"token",
		serde_urlencoded::to_string(encrypted).unwrap(),
	))
}
