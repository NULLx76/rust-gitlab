// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Delete a branch on a project.
#[derive(Debug, Builder, Clone)]
pub struct DeleteBranch<'a> {
    /// The project to create a branch on.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The name of the branch to delete.
    #[builder(setter(into))]
    branch: Cow<'a, str>,
}

impl<'a> DeleteBranch<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> DeleteBranchBuilder<'a> {
        DeleteBranchBuilder::default()
    }
}

impl<'a> Endpoint for DeleteBranch<'a> {
    fn method(&self) -> Method {
        Method::DELETE
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/repository/branches/{}", self.project, self.branch).into()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::projects::repository::branches::{DeleteBranch, DeleteBranchBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_is_necessary() {
        let err = DeleteBranch::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, DeleteBranchBuilderError, "project");
    }

    #[test]
    fn project_is_not_sufficient() {
        DeleteBranch::builder().project(1).build().unwrap();
    }

    #[test]
    fn project_and_branch_are_sufficient() {
        DeleteBranch::builder().project(1).branch("test").build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::DELETE)
            .endpoint("projects/simple%2Fproject/repository/branches/master")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = DeleteBranch::builder()
            .project("simple/project")
            .branch("master")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
