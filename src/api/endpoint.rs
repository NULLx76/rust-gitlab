// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::borrow::Cow;

use async_trait::async_trait;
use http::{self, header, Method, Request};
use reqwest::Url;
use serde::de::DeserializeOwned;

use crate::api::{
    query, ApiError, AsyncClient, AsyncQuery, BodyError, Client, Query, QueryParams, RestClient,
};

/// URL bases for endpoints.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum UrlBase {
    /// An endpoint uses the API v4 URL prefix.
    ApiV4,
    /// An endpoint acts on the instance itself.
    Instance,
}

impl UrlBase {
    /// Get the endpoint for a given URL base.
    pub fn endpoint_for<C>(&self, client: &C, endpoint: &str) -> Result<Url, ApiError<C::Error>>
    where
        C: RestClient,
    {
        match self {
            UrlBase::ApiV4 => client.rest_endpoint(endpoint),
            UrlBase::Instance => client.instance_endpoint(endpoint),
        }
    }
}

/// A trait for providing the necessary information for a single REST API endpoint.
pub trait Endpoint {
    /// The HTTP method to use for the endpoint.
    fn method(&self) -> Method;
    /// The path to the endpoint.
    fn endpoint(&self) -> Cow<'static, str>;

    /// The URL base of the API endpoint.
    fn url_base(&self) -> UrlBase {
        UrlBase::ApiV4
    }

    /// Query parameters for the endpoint.
    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }

    /// The body for the endpoint.
    ///
    /// Returns the `Content-Encoding` header for the data as well as the data itself.
    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        Ok(None)
    }
}

impl<E> Endpoint for &E
where
    E: Endpoint,
{
    fn method(&self) -> Method {
        (*self).method()
    }

    fn endpoint(&self) -> Cow<'static, str> {
        (*self).endpoint()
    }

    fn url_base(&self) -> UrlBase {
        (*self).url_base()
    }

    fn parameters(&self) -> QueryParams {
        (*self).parameters()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        (*self).body()
    }
}

impl<E, T, C> Query<T, C> for E
where
    E: Endpoint,
    T: DeserializeOwned,
    C: Client,
{
    fn query(&self, client: &C) -> Result<T, ApiError<C::Error>> {
        let mut url = self.url_base().endpoint_for(client, &self.endpoint())?;
        self.parameters().add_to_url(&mut url);

        let req = Request::builder()
            .method(self.method())
            .uri(query::url_to_http_uri(url));
        let (req, data) = if let Some((mime, data)) = self.body()? {
            let req = req.header(header::CONTENT_TYPE, mime);
            (req, data)
        } else {
            (req, Vec::new())
        };
        let rsp = client.rest(req, data)?;
        let status = rsp.status();
        let v = if let Ok(v) = serde_json::from_slice(rsp.body()) {
            v
        } else {
            return Err(ApiError::server_error(status, rsp.body()));
        };
        if !status.is_success() {
            return Err(ApiError::from_gitlab(v));
        } else if status == http::StatusCode::MOVED_PERMANENTLY {
            return Err(ApiError::moved_permanently(
                rsp.headers().get(http::header::LOCATION),
            ));
        }

        serde_json::from_value::<T>(v).map_err(ApiError::data_type::<T>)
    }
}

#[async_trait]
impl<E, T, C> AsyncQuery<T, C> for E
where
    E: Endpoint + Sync,
    T: DeserializeOwned + 'static,
    C: AsyncClient + Sync,
{
    async fn query_async(&self, client: &C) -> Result<T, ApiError<C::Error>> {
        let mut url = self.url_base().endpoint_for(client, &self.endpoint())?;
        self.parameters().add_to_url(&mut url);

        let req = Request::builder()
            .method(self.method())
            .uri(query::url_to_http_uri(url));
        let (req, data) = if let Some((mime, data)) = self.body()? {
            let req = req.header(header::CONTENT_TYPE, mime);
            (req, data)
        } else {
            (req, Vec::new())
        };
        let rsp = client.rest_async(req, data).await?;
        let status = rsp.status();
        let v = if let Ok(v) = serde_json::from_slice(rsp.body()) {
            v
        } else {
            return Err(ApiError::server_error(status, rsp.body()));
        };
        if !status.is_success() {
            return Err(ApiError::from_gitlab(v));
        } else if status == http::StatusCode::MOVED_PERMANENTLY {
            return Err(ApiError::moved_permanently(
                rsp.headers().get(http::header::LOCATION),
            ));
        }

        serde_json::from_value::<T>(v).map_err(ApiError::data_type::<T>)
    }
}

#[cfg(test)]
mod tests {
    use http::StatusCode;
    use serde::Deserialize;
    use serde_json::json;

    use crate::api::endpoint_prelude::*;
    use crate::api::{ApiError, AsyncQuery, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    struct Dummy;

    impl Endpoint for Dummy {
        fn method(&self) -> Method {
            Method::GET
        }

        fn endpoint(&self) -> Cow<'static, str> {
            "dummy".into()
        }
    }

    #[derive(Debug, Deserialize)]
    struct DummyResult {
        value: u8,
    }

    #[test]
    fn test_gitlab_non_json_response() {
        let endpoint = ExpectedUrl::builder().endpoint("dummy").build().unwrap();
        let client = SingleTestClient::new_raw(endpoint, "not json");

        let res: Result<DummyResult, _> = Dummy.query(&client);
        let err = res.unwrap_err();
        if let ApiError::GitlabService { status, .. } = err {
            assert_eq!(status, http::StatusCode::OK);
        } else {
            panic!("unexpected error: {}", err);
        }
    }

    #[test]
    fn test_gitlab_empty_response() {
        let endpoint = ExpectedUrl::builder().endpoint("dummy").build().unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let res: Result<DummyResult, _> = Dummy.query(&client);
        let err = res.unwrap_err();
        if let ApiError::GitlabService { status, .. } = err {
            assert_eq!(status, http::StatusCode::OK);
        } else {
            panic!("unexpected error: {}", err);
        }
    }

    #[test]
    fn test_gitlab_error_bad_json() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("dummy")
            .status(StatusCode::NOT_FOUND)
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let res: Result<DummyResult, _> = Dummy.query(&client);
        let err = res.unwrap_err();
        if let ApiError::GitlabService { status, .. } = err {
            assert_eq!(status, http::StatusCode::NOT_FOUND);
        } else {
            panic!("unexpected error: {}", err);
        }
    }

    #[test]
    fn test_gitlab_error_detection() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("dummy")
            .status(StatusCode::NOT_FOUND)
            .build()
            .unwrap();
        let client = SingleTestClient::new_json(
            endpoint,
            &json!({
                "message": "dummy error message",
            }),
        );

        let res: Result<DummyResult, _> = Dummy.query(&client);
        let err = res.unwrap_err();
        if let ApiError::Gitlab { msg } = err {
            assert_eq!(msg, "dummy error message");
        } else {
            panic!("unexpected error: {}", err);
        }
    }

    #[test]
    fn test_gitlab_error_detection_legacy() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("dummy")
            .status(StatusCode::NOT_FOUND)
            .build()
            .unwrap();
        let client = SingleTestClient::new_json(
            endpoint,
            &json!({
                "error": "dummy error message",
            }),
        );

        let res: Result<DummyResult, _> = Dummy.query(&client);
        let err = res.unwrap_err();
        if let ApiError::Gitlab { msg } = err {
            assert_eq!(msg, "dummy error message");
        } else {
            panic!("unexpected error: {}", err);
        }
    }

    #[test]
    fn test_gitlab_error_detection_unknown() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("dummy")
            .status(StatusCode::NOT_FOUND)
            .build()
            .unwrap();
        let err_obj = json!({
            "bogus": "dummy error message",
        });
        let client = SingleTestClient::new_json(endpoint, &err_obj);

        let res: Result<DummyResult, _> = Dummy.query(&client);
        let err = res.unwrap_err();
        if let ApiError::GitlabUnrecognized { obj } = err {
            assert_eq!(obj, err_obj);
        } else {
            panic!("unexpected error: {}", err);
        }
    }

    #[test]
    fn test_bad_deserialization() {
        let endpoint = ExpectedUrl::builder().endpoint("dummy").build().unwrap();
        let client = SingleTestClient::new_json(
            endpoint,
            &json!({
                "not_value": 0,
            }),
        );

        let res: Result<DummyResult, _> = Dummy.query(&client);
        let err = res.unwrap_err();
        if let ApiError::DataType { source, typename } = err {
            assert_eq!(source.to_string(), "missing field `value`");
            assert_eq!(typename, "gitlab::api::endpoint::tests::DummyResult");
        } else {
            panic!("unexpected error: {}", err);
        }
    }

    #[test]
    fn test_good_deserialization() {
        let endpoint = ExpectedUrl::builder().endpoint("dummy").build().unwrap();
        let client = SingleTestClient::new_json(
            endpoint,
            &json!({
                "value": 0,
            }),
        );

        let res: DummyResult = Dummy.query(&client).unwrap();
        assert_eq!(res.value, 0);
    }

    #[tokio::test]
    async fn test_good_deserialization_async() {
        let endpoint = ExpectedUrl::builder().endpoint("dummy").build().unwrap();
        let client = SingleTestClient::new_json(
            endpoint,
            &json!({
                "value": 0,
            }),
        );

        let res: DummyResult = Dummy.query_async(&client).await.unwrap();
        assert_eq!(res.value, 0);
    }
}
