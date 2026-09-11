use crate::utils::errors::ApiError;
use chrono::Local;
use once_cell::sync::Lazy;
use rand::distr::Alphanumeric;
use rand::{RngExt, rng};
use serde::Serialize;
use serde_json::Value;
use snowflake_me::Snowflake;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

// Atomic counter to ensure uniqueness within the same nanosecond
static COUNTER: AtomicU64 = AtomicU64::new(0);

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
    *SNOWFLAKE
        .next_id()
        .expect("failed to generate snowflake id")
}

pub fn gen_snowflake_slug() -> Result<(i64, String), Box<dyn std::error::Error>> {
    let id = gen_snowflake();
    let slug = base62_encode(id);
    Ok((id as i64, slug))
}

pub fn get_code(num: i16) -> String {
    let mut rng = rng();

    let code: String = (0..num)
        .map(|_| char::from_digit(rng.random_range(0..10), 10).unwrap())
        .collect();

    code
}

pub fn gen_string(size: usize) -> String {
    rng()
        .sample_iter(&Alphanumeric)
        .take(size)
        .map(char::from)
        .collect()
}

pub fn id_parser(val: &str, field: &str) -> Result<i64, ApiError> {
    val.parse::<i64>()
        .map_err(|_| ApiError::BadRequest(format!("Invalid {} format", field)))
}

pub fn get_serde_value<T: Serialize>(value: &Option<T>) -> Result<Option<Value>, ApiError> {
    value
        .as_ref()
        .map(|v| serde_json::to_value(v))
        .transpose()
        .map_err(|_| ApiError::InternalServerError)
}

fn luhn_check_digit(number: &str) -> u8 {
    let mut sum = 0;
    let mut double = true;

    for c in number.chars().rev() {
        let mut digit = c.to_digit(10).unwrap() as u8;

        if double {
            digit *= 2;
            if digit > 9 {
                digit -= 9;
            }
        }

        sum += digit as u32;
        double = !double;
    }

    ((10 - (sum % 10)) % 10) as u8
}

pub fn generate_account_number(branch_code: i64, customer_id: i64) -> String {
    let base = format!("{:06}{:08}", branch_code, customer_id);

    let check_digit = luhn_check_digit(&base);

    format!("{}{}", base, check_digit)
}

pub fn generate_reference_number(prefix: &str) -> String {
    let now = Local::now();

    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_nanos();

    // Get the last 8 digits of nanoseconds for uniqueness
    let time_part = (nanos % 100_000_000) as u64;

    let year = now.format("%y").to_string();

    // Get an atomic counter to ensure uniqueness if two calls happen in the same nanosecond
    let counter = COUNTER.fetch_add(1, Ordering::SeqCst);

    let unique_number = if time_part < 99_999_999 {
        let base = time_part * 100 + (counter % 100);
        format!("{:010}", base)
    } else {
        let val = (counter % 99_999_999) * 100 + (counter % 100);
        format!("{:010}", val)
    };

    let main_part = &unique_number[0..8];
    let year_part = &year[0..2];

    format!("{}{}{}", prefix, main_part, year_part)
}
