mod methods;
mod models;
mod tz_utils;

use gix::Repository;
use pyo3::prelude::*;

fn discover_repository(git_dir: String) -> anyhow::Result<Repository> {
    let repo = gix::discover(git_dir.clone().trim())?;
    Ok(repo)
}


/// A Python module implemented in Rust.
#[pymodule(gil_used = false)]
pub mod pygix_cartography {
    use super::*;

    #[pymodule(gil_used = true)]
    pub mod functions {
        // This is a submodule

        #[pymodule_export]
        use crate::methods::{get_diffs, traverse_commit_graph};
    }

    #[pymodule_init]
    fn init(#[allow(unused_variables)] m: &Bound<'_, PyModule>) -> PyResult<()> {
        pyo3_log::init();
        Ok(())
    }
}
