// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(clippy::module_inception)]

//! Runner-related API endpoints
//!
//! These endpoints are used for querying and modifying CI runners and their resources.

mod runners;

pub use self::runners::RunnerStatus;
pub use self::runners::RunnerType;
pub use self::runners::Runners;
pub use self::runners::RunnersBuilder;
pub use self::runners::RunnersBuilderError;
