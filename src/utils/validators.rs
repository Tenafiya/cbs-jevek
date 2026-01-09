use validator::ValidationError;

pub fn validate_operation(operand: &str) -> Result<(), ValidationError> {
    match operand {
        "TOGGLE" | "DELETE" => Ok(()),
        _ => Err(ValidationError::new("Invalid operation")),
    }
}
