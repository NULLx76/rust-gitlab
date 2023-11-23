use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Remove the fork relationship of a project
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct UnforkProject<'a> {
    /// The project to fork
    #[builder(setter(into))]
    project: NameOrId<'a>,
}

impl<'a> UnforkProject<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> UnforkProjectBuilder<'a> {
        UnforkProjectBuilder::default()
    }
}

impl<'a> Endpoint for UnforkProject<'a> {
    fn method(&self) -> Method {
        Method::DELETE
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/fork", self.project).into()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;
    

    use super::UnforkProject;
    use crate::{
        api::{projects::fork::UnforkProjectBuilderError, self, query::Query},
        test::client::{ExpectedUrl, SingleTestClient},
    };

    #[test]
    fn project_is_necessary() {
        let err = UnforkProject::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, UnforkProjectBuilderError, "project");
    }

    #[test]
    fn project_is_sufficient() {
        UnforkProject::builder().project("project").build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::DELETE)
            .endpoint("projects/simple%2Fproject/fork")
            .build()
            .unwrap();

        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = UnforkProject::builder()
            .project("simple/project")
            .build()
            .unwrap();

        api::ignore(endpoint).query(&client).unwrap();
    }
}
