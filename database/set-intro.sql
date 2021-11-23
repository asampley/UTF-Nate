insert into user_config (
	user_id, intro
) values (
	$1, $2
) on conflict (user_id) do
	update set intro = excluded.intro
;
