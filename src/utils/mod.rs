pub mod api_error;
pub mod hash;
pub mod jwt;
pub mod response;

// Utility function to format validation errors from the validator crate into a readable string format
pub fn format_validation_errors(errors: &validator::ValidationErrors) -> String {
    // Iterate through the field errors and collect them into a vector of strings
    let mut error_messages = Vec::new();
    // Loop through each field and its associated errors
    for (field, errors) in errors.field_errors() {
        for error in errors {
            let message = error
                .message
                .as_ref()
                .map(|m| m.to_string())
                .unwrap_or_else(|| "Validation error".to_string());
            error_messages.push(format!("{}: {}", field, message));
        }
    }
    // Join all error messages into a single string separated by commas
    error_messages.join(", ")
}
