GRANT ALTER ON KEYSPACE cycling TO coach;

GRANT SELECT ON ALL KEYSPACES TO team_manager;

GRANT EXECUTE ON FUNCTION cycling.fLog(double) TO team_manager;

GRANT ALL PERMISSIONS ON KEYSPACE cycling TO cycling_admin;

GRANT SELECT ON KEYSPACE cycling TO coach;

GRANT MODIFY ON KEYSPACE cycling TO coach;

GRANT AUTHORIZE ON ALL KEYSPACES TO sys_admin;

GRANT AUTHORIZE ON KEYSPACE cycling TO cycling_admin;

GRANT AUTHORIZE ON TABLE cycling.birthday_list TO sam;

GRANT ALL PERMISSIONS ON KEYSPACE cycling TO cycling_admin;

GRANT AUTHORIZE ON KEYSPACE cycling TO cycling_admin;

GRANT AUTHORIZE ON TABLE cycling.birthday_list TO sam;
