insert into guild_config (
	guild_id, volume_clip
) values (
	:guild_id, :volume_clip
) on conflict (guild_id) do
	update set volume_clip = excluded.volume_clip
;
