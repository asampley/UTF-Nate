use regex::{Regex, RegexBuilder};
use thiserror::Error;

use std::sync::LazyLock;

use crate::REQWEST_CLIENT;

use super::Result;
use super::api::{Artist, Playlist, PlaylistTracks, PlaylistTracksItem, Streamable, Track};

static PLAYLIST_EMBED_JSON: LazyLock<Regex> = LazyLock::new(|| {
	RegexBuilder::new(
		r#"<script\s+([^>]*("[^"]*")?)*type="application/json"([^>]*("[^"]*")*)*>(?<json>.*)</script>"#,
	)
	.dot_matches_new_line(true)
	.multi_line(true)
	.build()
	.unwrap()
});

#[derive(Debug, Error)]
pub enum Error {
	#[error("could not find json in embed")]
	MissingJson(String),
	#[error("failed to parse json in embed: {0}")]
	ParseJson(String),
	#[error("could not find tracks in embed json")]
	Tracks,
	#[error("could not find track id in embed json")]
	TrackId,
	#[error("could not find track name in embed json")]
	TrackName,
	#[error("could not find track artists in embed json")]
	TrackArtists,
}

pub async fn playlist(playlist_id: &str) -> Result<Playlist> {
	let embed = REQWEST_CLIENT
		.get(format!(
			"https://open.spotify.com/embed/playlist/{}",
			playlist_id
		))
		.send()
		.await?
		.text()
		.await?;

	scrape_playlist_embed(embed)
}

fn scrape_playlist_embed(embed: String) -> Result<Playlist> {
	let Some(json_text) = PLAYLIST_EMBED_JSON
		.captures(&embed)
		.and_then(|c| c.name("json").map(|v| v.as_str()))
	else {
		return Err(Error::MissingJson(embed).into());
	};

	let json: serde_json::Value =
		serde_json::from_str(json_text).map_err(|_| Error::ParseJson(json_text.to_owned()))?;

	let name = json
		.pointer("/props/pageProps/state/data/entity/name")
		.and_then(|v| v.as_str());

	let tracks = json
		.pointer("/props/pageProps/state/data/entity/trackList")
		.inspect(|v| println!("{}", v))
		.and_then(|v| v.as_array())
		.ok_or(Error::Tracks)?
		.iter()
		.map(|entry| {
			let id = entry
				.get("uri")
				.and_then(|uri| uri.as_str()?.strip_prefix("spotify:track:"))
				.ok_or(Error::TrackId)?
				.to_owned();

			let name = entry
				.get("title")
				.and_then(|i| i.as_str())
				.ok_or(Error::TrackName)?
				.to_owned();
			let duration_ms = entry.get("duration").and_then(|i| i.as_u64()).unwrap_or(0);
			let artists = entry
				.get("subtitle")
				.and_then(|v| v.as_str())
				.ok_or(Error::TrackArtists)?
				.split(',')
				.map(|artist| Artist {
					id: String::default(),
					name: artist.trim().to_owned(),
				})
				.collect();

			Ok(PlaylistTracksItem {
				track: Track {
					id,
					name,
					duration_ms,
					artists,
				},
			})
		})
		.collect::<Result<Vec<_>>>()?;

	Ok(Playlist {
		name: name.unwrap_or("Unknown").to_string(),
		tracks: Streamable {
			next: None,
			rest: PlaylistTracks { items: tracks },
		},
	})
}

#[cfg(test)]
mod test {
	use super::*;

	const TEST_DOC: &str = r#"<script id="__NEXT_DATA__" type="application/json">{"props":{"pageProps":{"state":{"data":{"entity":{"type":"playlist","name":"Chill Mix","uri":"spotify:playlist:37i9dQZF1EVHGWrwldPRtj","id":"37i9dQZF1EVHGWrwldPRtj","title":"Chill Mix","subtitle":"Spotify","coverArt":{"sources":[{"height":null,"width":null,"url":"https://seed-mix-image.spotifycdn.com/v6/img/chill/6FXMGgJwohJLUSr5nVlf9X/en/default"}]},"releaseDate":null,"duration":0,"maxDuration":0,"isPlayable":true,"isExplicit":false,"hasVideo":false,"relatedEntityUri":"spotify:playlist:37i9dQZF1EVHGWrwldPRtj","trackList":[{"uri":"spotify:track:7uv632EkfwYhXoqf8rhYrg","uid":"62383661636530303831643736323638","title":"Angel","subtitle":"Massive Attack, Horace Andy","isExplicit":false,"duration":379533,"isPlayable":true,"audioPreview":{"format":"MP3_96","url":"https://p.scdn.co/mp3-preview/bf35426762db02d161ca834044d48ab355fc97e2"}}],"visualIdentity":{"backgroundBase":{"alpha":255,"blue":56,"green":64,"red":56},"backgroundTintedBase":{"alpha":255,"blue":86,"green":95,"red":86},"textBase":{"alpha":255,"blue":255,"green":255,"red":255},"textBrightAccent":{"alpha":255,"blue":255,"green":255,"red":255},"textSubdued":{"alpha":255,"blue":198,"green":208,"red":198},"image":[]}},"embeded_entity_uri":"spotify:playlist:37i9dQZF1EVHGWrwldPRtj","defaultAudioFileObject":{"passthrough":"NONE"}},"settings":{"rtl":false,"session":{"accessToken":"BQAKR5QwaVPKdgiYrnBhxNIQjYnbHrNAeLBWtAo6TEO0uK8i57E0nLrDvm7cqBOz1UDUHcgt3p3zP3VHmJs07kIJoT1C-u2StS1sfEDc8Tb0sMGHwCI2hhfjDIXpVSYhNqXpkby6PuLBUh5g0nFLpiw0TAHci-gefcaqfcXiMDbo_7K75RN5wN_uOc8ujCo5jLQBo_634Mt60rwl_8MR2OjQbzkq_UlO4X9DfD1kOs07-5bi","accessTokenExpirationTimestampMs":1734396141267,"isAnonymous":false},"entityContext":"playlist","clientId":"ab9ad0d96a624805a7d51e8868df1f97","isMobile":false,"isSafari":false,"isIOS":false,"isTablet":false,"isDarkMode":false}},"config":{"correlationId":"a86c86e3c7f837544ff48316e71e961b","strings":{"en":{"translation":{}}},"locale":"en","clientId":"ab9ad0d96a624805a7d51e8868df1f97","restrictionId":""},"_sentryTraceData":"c955ce83c2e739440fec7e083a9761ef-f2e59045ad0c9d37-0","_sentryBaggage":"sentry-environment=production,sentry-release=476687185d39b1fe23fe421409df6473846a98b5,sentry-public_key=4cc707ab12ea4779b417479c0550a5cb,sentry-trace_id=c955ce83c2e739440fec7e083a9761ef,sentry-sampled=false"},"__N_SSP":true},"page":"/playlist/[id]","query":{"id":"37i9dQZF1EVHGWrwldPRtj"},"buildId":"13f547fa-fb58-4933-9194-c7c60dfac45d","assetPrefix":"https://embed-cdn.spotifycdn.com","isFallback":false,"gssp":true,"scriptLoader":[]}</script>"#;

	#[test]
	fn test_regex() {
		assert!(PLAYLIST_EMBED_JSON.is_match_at(TEST_DOC, 0))
	}

	#[test]
	fn test_scrape() {
		let playlist = scrape_playlist_embed(TEST_DOC.to_owned()).unwrap();

		assert!(playlist.name == "Chill Mix");
		assert!(playlist.tracks.rest.items.len() == 1)
	}
}
