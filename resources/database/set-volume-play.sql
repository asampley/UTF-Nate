insert into guild_config (
	guild_id, volume_play
) values (
	$1, $2
) on conflict (guild_id) do
	update set volume_play = excluded.volume_play
;
