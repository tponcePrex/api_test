pub type UserIdType = u16;
pub type UserScoreType = u16;

#[allow(dead_code)]
pub enum ResponseCodes {
    Ok,
    NoContent,
    NotModified,
    BadRequest,
    NotFound,
    InternalServerError,
    Unauthorized
}