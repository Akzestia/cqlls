/*
    MIT License

    Copyright (c) 2026 アクゼスティア
*/

const VERSION_MAJOR: u8 = 4;
const VERSION_MINOR: u8 = 0;
const VERSION_PATCH: u8 = 1;

pub fn version() -> String {
    format!("cqlls v{VERSION_MAJOR}.{VERSION_MINOR}.{VERSION_PATCH}")
}
