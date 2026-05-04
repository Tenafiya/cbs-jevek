use sea_orm::prelude::Decimal;

pub fn major_conversion(price: i64, currency: &str) -> Decimal {
    let decimal_places = match currency.to_uppercase().as_str() {
        "JPY" | "KRW" | "CLP" | "ISK" => 0,
        "BHD" | "IQD" | "JOD" | "KWD" | "LYD" | "OMR" | "TND" => 3,
        _ => 2,
    };

    let divisor = Decimal::from(10_i64.pow(decimal_places));
    
    Decimal::from(price) / divisor
}

pub fn minor_conversion(price: Decimal, currency: &str) -> i64 {
    let decimal_places = match currency.to_uppercase().as_str() {
        "JPY" | "KRW" | "CLP" | "ISK" => 0,
        "BHD" | "IQD" | "JOD" | "KWD" | "LYD" | "OMR" | "TND" => 3,
        _ => 2,
    };

    let multiplier = Decimal::from(10_i64.pow(decimal_places));

    let minor = (price * multiplier).round();

    minor.to_string().parse::<i64>().unwrap_or(0)
}