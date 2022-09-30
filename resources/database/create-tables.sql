create table if not exists user_config (
	user_id bigint primary key,
	intro text,
	outro text
);

create table if not exists guild_config (
	guild_id bigint primary key,
	bot_intro text,
	volume_clip real check (volume_clip >= 0.0 and volume_clip <= 1.0),
	volume_play real check (volume_play >= 0.0 and volume_play <= 1.0)
);
