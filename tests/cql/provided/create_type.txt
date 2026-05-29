CREATE TYPE IF NOT EXISTS cycling.basic_info (
    birthday timestamp,
    nationality text,
    height text,
    weight text
);

CREATE TYPE IF NOT EXISTS cycling.basic_info_expire (
    birthday timestamp,
    nationality text,
    height text,
    weight text,
    next_race text
);
