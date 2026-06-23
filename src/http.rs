use axum::routing::*;

use tower_http::services::ServeDir;
use tracing::info;

use crate::commands::http::{FormRouter, command_list};
use crate::commands::*;
use crate::configuration::HttpConfig;

use chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};

use serenity::model::prelude::{GuildId, UserId};

/// Token that is used for the web interface.
///
/// Contains details of how the command was called.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Token {
	/// Guild id for the command, which is `None` when there is no guild.
	pub guild_id: Option<GuildId>,

	/// User id that invoked the command. Must always be set.
	pub user_id: UserId,

	/// Expiry timestamp for token
	pub expiry: DateTime<Utc>,
}

impl Token {
	pub fn is_expired(&self) -> bool {
		self.expiry < Utc::now()
	}
}

pub async fn axum_task(http_config: &HttpConfig, state: BotState) -> Result<(), std::io::Error> {
	info!("Starting HTTP server");

	let mut commands = Vec::new();

	crate::commands::for_each_recursive(|command| {
		if command.identifying_name != "token" {
			commands.push(command);
		}
	});

	let app = axum::Router::new()
		.route("/", get(command_list))
		.form_route(&help::poise::help(), help::http::help)
		// TODO consider ability to apply restrictions on these commands
		//.form_route(&external::poise::cmd(), external::http::cmd)
		//.form_route(&external::poise::cmdlist(), external::http::cmdlist)
		.form_route(&join::poise::summon(), join::http::summon)
		.form_route(&join::poise::banish(), join::http::banish)
		.form_route(&herald::poise::intro(), herald::http::intro)
		.form_route(&herald::poise::introbot(), herald::http::introbot)
		.form_route(&herald::poise::outro(), herald::http::outro)
		.form_route(&play::poise::clip(), play::http::clip)
		.form_route(&play::poise::play(), play::http::play)
		.form_route(&play::poise::playnext(), play::http::playnext)
		.form_route(&play::poise::playnow(), play::http::playnow)
		.form_route(&queue::poise::stop(), queue::http::stop)
		.form_route(&queue::poise::skip(), queue::http::skip)
		.form_route(&queue::poise::pause(), queue::http::pause)
		.form_route(&queue::poise::unpause(), queue::http::unpause)
		.form_route(&queue::poise::queue(), queue::http::queue)
		.form_route(&queue::poise::shuffle(), queue::http::shuffle)
		.form_route(&queue::poise::shufflenow(), queue::http::shufflenow)
		.form_route(&queue::poise::r#loop(), queue::http::r#loop)
		.form_route(&queue::poise::r#move(), queue::http::r#move)
		.form_route(&voice::poise::list(), voice::http::list)
		.form_route(&voice::poise::volume(), voice::http::volume)
		.form_route(&voice::poise::volume_get(), voice::http::volume_get)
		.form_route(&voice::poise::volume_clip(), voice::http::volume_clip)
		.form_route(&voice::poise::volume_play(), voice::http::volume_play)
		.form_route(&voice::poise::volume_now(), voice::http::volume_now)
		.form_route(&unicode::poise::unicode(), unicode::http::unicode)
		.form_route(&roll::poise::roll(), roll::http::roll)
		.route("/token", get(token::http::token))
		.fallback_service(ServeDir::new("resources/web"))
		.with_state(state);

	let listener = tokio::net::TcpListener::bind(http_config.listen)
		.await
		.unwrap();

	axum::serve(listener, app).await
}
