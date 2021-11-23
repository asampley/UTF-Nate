insert into guild_config (
	guild_id, volume_play
) values (
	:guild_id, :volume_play
) on conflict (guild_id) do
	update set volume_play = excluded.volume_play
;
