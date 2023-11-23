use derive_builder::Builder;

use crate::api::common::{NameOrId, VisibilityLevel};
use crate::api::endpoint_prelude::*;

/// Fork a project
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct ForkProject<'a> {
    /// The project to fork
    #[builder(setter(into))]
    project: NameOrId<'a>,

    /// The name of the project.
    #[builder(setter(into), default)]
    name: Option<Cow<'a, str>>,
    /// The path of the project.
    #[builder(setter(into), default)]
    path: Option<Cow<'a, str>>,

    /// The description of the new project.
    #[builder(setter(into), default)]
    description: Option<Cow<'a, str>>,

    /// For forked projects, target merge requests to this project. If false, the target is the upstream project.
    #[builder(default)]
    mr_default_target_self: Option<bool>,

    /// The ID of the namespace that the project is forked to
    #[builder(default)]
    namespace_id: Option<u64>,

    /// The path of the namespace that the project is forked to.
    #[builder(default)]
    namespace_path: Option<Cow<'a, str>>,

    /// Visibility of the new repo
    #[builder(default)]
    visibility: Option<VisibilityLevel>,

    /// Select a specfic branch to fork, empty is all
    #[builder(default)]
    branch: Option<Cow<'a, str>>,
}

impl<'a> ForkProject<'a> {
    /// Create a builder for the endpoint
    pub fn builder() -> ForkProjectBuilder<'a> {
        ForkProjectBuilder::default()
    }
}

impl<'a> Endpoint for ForkProject<'a> {
    fn method(&self) -> http::Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/fork", self.project).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = FormParams::default();

        params
            .push_opt("description", self.description.as_ref())
            .push_opt("mr_default_target_self", self.mr_default_target_self)
            .push_opt("name", self.name.as_ref())
            .push_opt("namespace_id", self.namespace_id)
            .push_opt("namespace_path", self.namespace_path.as_ref())
            .push_opt("path", self.path.as_ref())
            .push_opt("visibility", self.visibility)
            .push_opt("branches", self.branch.as_ref());

        params.into_body()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::{
        api::{
            self,
            projects::fork::{ForkProject, ForkProjectBuilderError},
            Query,
        },
        test::client::{ExpectedUrl, SingleTestClient},
    };

    #[test]
    fn project_is_needed() {
        let err = ForkProject::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, ForkProjectBuilderError, "project");
    }

    #[test]
    fn project_is_sufficient() {
        ForkProject::builder().project("project").build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/fork")
            .content_type("application/x-www-form-urlencoded")
            .body_str("")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ForkProject::builder()
            .project("simple/project")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_name() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/fork")
            .content_type("application/x-www-form-urlencoded")
            .body_str("name=name")
            .build()
            .unwrap();

        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ForkProject::builder()
            .project("simple/project")
            .name("name")
            .build()
            .unwrap();

        api::ignore(endpoint).query(&client).unwrap();
    }
}
