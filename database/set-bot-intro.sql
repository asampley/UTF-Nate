insert into guild_config (
	guild_id, bot_intro
) values (
	$1, $2
) on conflict (guild_id) do
	update set bot_intro = excluded.bot_intro
;
