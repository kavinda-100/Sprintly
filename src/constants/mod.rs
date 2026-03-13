#[allow(dead_code)]
pub const JWT_EXPIRATION_DAYS: i64 = 7;

#[allow(dead_code)]
// This constant can be used as a fallback for JWT expiration time in milliseconds if needed (7 days in milliseconds)
pub const JWT_EXPIRATION_DAYS_FALLBACK_IN_MILLISECONDS: usize =
    (JWT_EXPIRATION_DAYS * 24 * 3600 * 1000) as usize; // 7 days in milliseconds
