insert into user_config (
	user_id, outro
) values (
	$1, $2
) on conflict (user_id) do
	update set outro = excluded.outro
;
