CREATE TABLE cycling.cyclist_name (
    id               UUID PRIMARY KEY,
    lastname         text,
    firstname        text
);

CREATE TABLE cycling.cyclist_name (
    id               UUID,
    lastname         text,
    firstname        text,

    PRIMARY KEY (id)
);

CREATE TABLE cycling."addThis" (
    a        int PRIMARY KEY
);

CREATE TABLE IF NOT EXISTS cycling.rank_by_year_and_name (
    race_year           int,
    race_name           text,
    cyclist_name        text,
    rank                int,

    PRIMARY KEY ((race_year, race_name), rank)
);

CREATE TABLE IF NOT EXISTS cycling.cyclist_category (
    category        text,
    points          int,
    id              UUID,
    lastname        text,

    PRIMARY KEY (category, points)
)
WITH CLUSTERING ORDER BY (points DESC);

CREATE TABLE IF NOT EXISTS cycling.race_winners (
    cyclist_name         FROZEN<fullname>,
    yclist_name          frozen<fullname>
    race_name            text,
    race_position        int,

    PRIMARY KEY (race_name, race_position)
);

CREATE TABLE cycling.geospatial (
    id         text PRIMARY KEY,
    point      'PointType',
    linestring 'LineStringType'
);

CREATE TABLE IF NOT EXISTS cycling.cyclist_id (
    lastname         text,
    firstname        text,
    age              int,
    id               UUID,

    PRIMARY KEY ((lastname, firstname), age)
);

CREATE TABLE IF NOT EXISTS cycling.cyclist_category (
    category        text,
    points          int,
    id              UUID,
    lastname        text,

    PRIMARY KEY (category, points)
)
WITH CLUSTERING ORDER BY (points DESC);

CREATE TABLE IF NOT EXISTS cycling.cyclist_emails (
    userid        text PRIMARY KEY,
    id            UUID,
    emails        set<text>
) WITH ID =       '1bb7516e-b140-11e8-96f8-529269fb1459';
