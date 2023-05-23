use chrono::{
    DateTime,
    Utc,
};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub(crate) struct Response<T, E> {
    pub(crate) timestamp: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) error: Option<E>,
}

impl<T, E> Response<T, E> {
    pub(crate) fn from_result(result: Result<T, E>) -> Self {
        let (data, error) = match result {
            Ok(data) => (Some(data), None),
            Err(error) => (None, Some(error)),
        };

        Response {
            timestamp: Utc::now(),
            data,
            error,
        }
    }
}

#[cfg(test)]
mod test {
    use chrono::Utc;

    use super::Response;

    #[test]
    fn ok_response_has_timestamp_of_creation() {
        let now = Utc::now();
        let response: Response<i8, ()> = Response::from_result(Ok(1));
        let duration = now - response.timestamp;
        assert!(duration.num_seconds().abs() < 1);
    }

    #[test]
    fn err_response_has_timestamp_of_creation() {
        let now = Utc::now();
        let response: Response<(), i8> =
            Response::from_result(Err(1));
        let duration = now - response.timestamp;
        assert!(duration.num_seconds().abs() < 1);
    }

    #[test]
    fn make_response_from_ok_result() {
        let result: Result<i8, ()> = Ok(1);
        let response = Response::from_result(result);
        assert!(response.data.is_some());
        assert!(response.error.is_none());
    }

    #[test]
    fn make_response_from_err_result() {
        let result: Result<(), i8> = Err(1);
        let response = Response::from_result(result);
        assert!(response.data.is_none());
        assert!(response.error.is_some());
    }
}
