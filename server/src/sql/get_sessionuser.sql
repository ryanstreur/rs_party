select 
    s.session_key, 
    u.id as user_id,
    s.session_data,
    s.created,
    s.updated,
    u.email_address,
    u.name,
    u.is_superuser
from user u
    join session s on u.id = s.user_id
where s.session_key = $1
order by s.created desc, s.updated desc
limit 1;