use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ApiError {
    pub code: u16,
    pub message: String,
}

impl ApiError {
    pub fn new(code: u16, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiResponse<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<ApiError>>,
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        let status = self
            .status
            .and_then(|s| StatusCode::from_u16(s).ok())
            .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

        if status == StatusCode::NO_CONTENT {
            return status.into_response();
        }
        
        (status, Json(self)).into_response()
    }
}

impl<T> ApiResponse<T> {
    pub fn new(
        status: Option<u16>,
        message: Option<String>,
        data: Option<T>,
        errors: Option<Vec<ApiError>>,
    ) -> Self {
        Self {
            status,
            message,
            data,
            errors,
        }
    }

    pub fn ok(data: T) -> Self {
        Self {
            status: Some(200),
            message: Some("Success".to_string()),
            data: Some(data),
            errors: None,
        }
    }

    pub fn created(data: T) -> Self {
        Self {
            status: Some(201),
            message: Some("Created successfully.".to_string()),
            data: Some(data),
            errors: None,
        }
    }

    pub fn no_content() -> Self {
        Self {
            status: Some(204),
            message: None,
            data: None,
            errors: None,
        }
    }

    pub fn error(code: u16, message: impl Into<String>, error_code: String) -> Self {
        let message = message.into();
        Self {
            status: Some(code),
            message: Some(error_code),
            data: None,
            errors: Some(vec![ApiError::new(code, message)]),
        }
    }

    pub fn with_errors_status(status: u16, errors: Vec<ApiError>) -> Self {
        Self {
            status: Some(status),
            message: None,
            data: None,
            errors: Some(errors),
        }
    }

    pub fn with_errors(errors: Vec<ApiError>) -> Self {
        Self {
            status: Some(400),
            message: None,
            data: None,
            errors: Some(errors),
        }
    }

    pub fn map<U, F>(self, f: F) -> ApiResponse<U>
    where
        F: FnOnce(T) -> U,
    {
        ApiResponse {
            status: self.status,
            message: self.message,
            data: self.data.map(f),
            errors: self.errors,
        }
    }

    pub fn into_result(self) -> Result<T, Vec<ApiError>> {
        match (self.data, self.errors) {
            (Some(d), None) => Ok(d),
            (_, Some(errs)) => Err(errs),
            (None, None) => Err(vec![ApiError::new(
                self.status.unwrap_or(0),
                self.message.unwrap_or_else(|| "no data".to_string()),
            )]),
        }
    }

    pub fn is_success(&self) -> bool {
        match self.status {
            Some(s) => (200..300).contains(&s),
            None => false,
        }
    }

    pub fn is_client_error(&self) -> bool {
        match self.status {
            Some(s) => (400..500).contains(&s),
            None => false,
        }
    }

    pub fn is_server_error(&self) -> bool {
        match self.status {
            Some(s) => (500..600).contains(&s),
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, to_value, Value};

    #[test]
    fn serialize_ok_response() {
        let resp = ApiResponse::ok("payload");
        let v: Value = to_value(&resp).expect("serialize");
        assert_eq!(v["status"], json!(200));
        assert_eq!(v["message"], json!("Success"));
        assert_eq!(v["data"], json!("payload"));
        assert!(v.get("errors").is_none());
    }

    #[test]
    fn serialize_no_content() {
        let resp: ApiResponse<()> = ApiResponse::no_content();
        let v = serde_json::to_value(&resp).unwrap();
        assert_eq!(v["status"], json!(204));
        assert_eq!(v["message"], json!("No Content"));
        assert!(v.get("data").is_none());
        assert!(v.get("errors").is_none());
    }

    #[test]
    fn error_into_result() {
        let resp: ApiResponse<String> =
            ApiResponse::error(404, "Not found", "NOT_FOUND".to_string());
        assert!(resp.is_client_error());
        let res = resp.into_result();
        assert!(res.is_err());
        let errs = res.err().unwrap();
        assert_eq!(errs.len(), 1);
        assert_eq!(errs[0], ApiError::new(404, "Not found"));
    }

    #[test]
    fn map_data() {
        let resp = ApiResponse::ok(2);
        let mapped = resp.map(|n| n * 3);
        assert_eq!(mapped.data, Some(6));
        assert!(mapped.is_success());
    }
}
