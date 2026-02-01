use chrono::{Datelike, NaiveDate, Utc};
use sea_orm::prelude::Decimal;
use validator::ValidationError;

pub fn validate_operation(operand: &str) -> Result<(), ValidationError> {
    match operand {
        "TOGGLE" | "DELETE" => Ok(()),
        _ => Err(ValidationError::new("Invalid operation")
            .with_message("Operation must be TOGGLE or DELETE".into())),
    }
}

pub fn validate_gender(gender: &str) -> Result<(), ValidationError> {
    match gender {
        "MALE" | "FEMALE" => Ok(()),
        _ => Err(ValidationError::new("Invalid gender")
            .with_message("Gender must be MALE or FEMALE".into())),
    }
}

pub fn validate_date(birth_date: &NaiveDate) -> Result<(), ValidationError> {
    let today = Utc::now().date_naive();

    if *birth_date >= today {
        return Err(ValidationError::new("birth_date_in_future")
            .with_message("Birth date must be in the past".into()));
    }

    let mut age = today.year() - birth_date.year();

    if today.ordinal() < birth_date.ordinal() {
        age -= 1;
    }

    if age < 18 {
        return Err(
            ValidationError::new("underage").with_message("Must be at least 18 years old".into())
        );
    }

    Ok(())
}

pub fn validate_income(value: &Decimal) -> Result<(), ValidationError> {
    let min = Decimal::ZERO;
    let max = Decimal::new(1_000_000_0000, 4);

    if *value < min || *value > max {
        return Err(ValidationError::new("monthly_income_range")
            .with_message("Monthly income cannot be < 0.0000 and > 1000000.0000".into()));
    }

    Ok(())
}
