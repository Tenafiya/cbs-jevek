use crate::utils::errors::ApiError;
use once_cell::sync::Lazy;
use rand::distr::Alphanumeric;
use rand::{Rng, rng};
use snowflake_me::Snowflake;

pub const BASE62: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

static SNOWFLAKE: Lazy<Snowflake> = Lazy::new(|| {
    Snowflake::builder()
        .bit_len_time(41)
        .bit_len_sequence(12)
        .bit_len_data_center_id(5)
        .bit_len_machine_id(5)
        .machine_id(&|| Ok(15))
        .data_center_id(&|| Ok(7))
        .finalize()
        .expect("failed to initialize snowflake generator")
});

pub fn base62_encode(mut num: u64) -> String {
    if num == 0 {
        return "0".to_string();
    }

    let mut buf = Vec::with_capacity(11);

    while num > 0 {
        let rem = (num % 62) as usize;
        buf.push(BASE62[rem]);
        num /= 62;
    }

    buf.reverse();

    unsafe { String::from_utf8_unchecked(buf) }
}

pub fn gen_snowflake() -> u64 {
    SNOWFLAKE
        .next_id()
        .expect("failed to generate snowflake id")
}

pub fn gen_snowflake_slug() -> Result<(i64, String), Box<dyn std::error::Error>> {
    let id = gen_snowflake();
    let slug = base62_encode(id);
    Ok((id as i64, slug))
}

pub async fn get_code(num: i16) -> String {
    let mut rng = rng();

    let code: String = (0..num)
        .map(|_| char::from_digit(rng.random_range(0..10), 10).unwrap())
        .collect();

    code
}

pub async fn gen_string(size: usize) -> String {
    rng()
        .sample_iter(&Alphanumeric)
        .take(size)
        .map(char::from)
        .collect()
}

pub async fn id_parser(val: &str, field: &str) -> Result<i64, ApiError> {
    val.parse::<i64>()
        .map_err(|_| ApiError::BadRequest(format!("Invalid {} format", field)))
}
