use lambda_http::{http::StatusCode, Response};

pub struct ApiHelper;

impl ApiHelper {
    pub fn response(
        status_code: StatusCode,
        body: String,
        content_type: String,
    ) -> Response<String> {
        Response::builder()
            .status(status_code)
            .header("Content-Type", content_type)
            .body(body)
            .unwrap()
    }
}
