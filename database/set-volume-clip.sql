insert into guild_config (
	guild_id, volume_clip
) values (
	$1, $2
) on conflict (guild_id) do
	update set volume_clip = excluded.volume_clip
;
