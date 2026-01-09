use rand::{Rng, rng};
use snowflake_me::Snowflake;

pub const BASE62: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

pub fn base62_encode(mut num: u64) -> String {
    if num == 0 {
        return "0".to_string();
    }

    let mut buf = Vec::new();

    while num > 0 {
        let rem = (num % 62) as usize;
        buf.push(BASE62[rem]);
        num /= 62;
    }

    buf.reverse();
    String::from_utf8(buf).unwrap()
}

pub fn gen_snowflake() -> Result<u64, Box<dyn std::error::Error>> {
    let bit_len_time = 41;
    let bit_len_sequence = 12;
    let bit_len_data_center_id = 5;
    let bit_len_machine_id = 5;

    let sf = Snowflake::builder()
        .bit_len_time(bit_len_time)
        .bit_len_sequence(bit_len_sequence)
        .bit_len_data_center_id(bit_len_data_center_id)
        .bit_len_machine_id(bit_len_machine_id)
        .machine_id(&|| Ok(15))
        .data_center_id(&|| Ok(7))
        .finalize()?;

    let id = sf.next_id()?;

    Ok(id)
}

pub fn gen_snowflake_slug() -> Result<(i64, String), Box<dyn std::error::Error>> {
    let id = gen_snowflake()?;
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
