CREATE MATERIALIZED VIEW IF NOT EXISTS cycling.cyclist_by_age AS
SELECT age, cid, birthday, country, name
FROM cycling.cyclist_base
WHERE age IS NOT NULL AND cid IS NOT NULL

PRIMARY KEY (age, cid)
WITH CLUSTERING ORDER BY (cid ASC)
AND caching = {
    'keys' : 'ALL',
    'rows_per_partition' : '100'
    }
AND comment = 'Based on table cyclist_base';

CREATE MATERIALIZED VIEW IF NOT EXISTS cycling.cyclist_by_birthday
AS SELECT age, cid, birthday, name, country
FROM cycling.cyclist_base
WHERE birthday IS NOT NULL AND cid IS NOT NULL

PRIMARY KEY (birthday, cid);

CREATE MATERIALIZED VIEW IF NOT EXISTS cycling.cyclist_by_country
AS SELECT age, cid, birthday, name, country
FROM cycling.cyclist_base
WHERE country IS NOT NULL AND cid IS NOT NULL
PRIMARY KEY (country, cid);
