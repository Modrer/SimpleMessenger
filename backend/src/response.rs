use rocket::response::Responder;

#[derive(Responder)]
pub enum AuthorizeResponder {
    Ok(String),
    #[response(status = 401, content_type = "application/json")]
    Unauthorized(()),
}
#[derive(Responder)]
pub enum RegistrationResult {
    #[response(status = 200, content_type = "application/json")]
    Ok(()),
    #[response(status = 500, content_type = "application/json")]
    Err(()),
}
#[derive(Responder)]
pub enum Response {
    #[response(status = 200, content_type = "application/json")]
    Ok(String),
    #[response(status = 500, content_type = "application/json")]
    Err(()),
}
