// Copyright (c) 2018 The predicates-rs Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::path;

use Predicate;

/// Predicate that checks if a file is present
///
/// This is created by the `predicate::path::exists` and `predicate::path::missing`.
#[derive(Debug)]
pub struct ExistencePredicate {
    exists: bool,
}

impl Predicate for ExistencePredicate {
    type Item = path::Path;

    fn eval(&self, path: &path::Path) -> bool {
        path.exists() == self.exists
    }
}

/// Creates a new `Predicate` that ensures the path exists.
///
/// # Examples
///
/// ```
/// use std::path::Path;
/// use predicates::predicate::*;
///
/// let predicate_fn = path::exists();
/// assert_eq!(true, predicate_fn.eval(Path::new("Cargo.toml")));
/// ```
pub fn exists() -> ExistencePredicate {
    ExistencePredicate { exists: true }
}

/// Creates a new `Predicate` that ensures the path doesn't exist.
///
/// # Examples
///
/// ```
/// use std::path::Path;
/// use predicates::predicate::*;
///
/// let predicate_fn = path::missing();
/// assert_eq!(true, predicate_fn.eval(Path::new("non-existent-file.foo")));
/// ```
pub fn missing() -> ExistencePredicate {
    ExistencePredicate { exists: false }
}
