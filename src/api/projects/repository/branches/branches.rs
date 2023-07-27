// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Query for a specific branch in a project.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct Branches<'a> {
    /// The project to get a branch from.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// Filter branches by a search query.
    ///
    /// The `^` and `$` anchors are supported to search for "starts with" and "ends with"
    /// operators.
    #[builder(setter(into), default)]
    search: Option<Cow<'a, str>>,
    /// Filter branches matching an [`re2`][re2] regular expression.
    ///
    /// [re2]: https://github.com/google/re2/wiki/Syntax
    #[builder(setter(into), default)]
    regex: Option<Cow<'a, str>>,
}

impl<'a> Branches<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> BranchesBuilder<'a> {
        BranchesBuilder::default()
    }
}

impl<'a> Endpoint for Branches<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/repository/branches", self.project).into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();

        params
            .push_opt("search", self.search.as_ref())
            .push_opt("regex", self.regex.as_ref());

        params
    }
}

impl<'a> Pageable for Branches<'a> {}

#[cfg(test)]
mod tests {
    use crate::api::projects::repository::branches::{Branches, BranchesBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_is_necessary() {
        let err = Branches::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, BranchesBuilderError, "project");
    }

    #[test]
    fn project_is_sufficient() {
        Branches::builder().project(1).build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/repository/branches")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Branches::builder()
            .project("simple/project")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_search() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/repository/branches")
            .add_query_params(&[("search", "query")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Branches::builder()
            .project("simple/project")
            .search("query")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_regex() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/repository/branches")
            .add_query_params(&[("regex", "query")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Branches::builder()
            .project("simple/project")
            .regex("query")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
