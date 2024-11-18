pub mod navigator;
pub mod window;

pub(crate) mod opfs {
    mod fs;
    mod opfs_dir;
    mod opfs_file;

    pub(crate) use opfs_dir::OpfsDir;
    pub(crate) use opfs_file::OpfsFile;
    #[allow(unused_imports)]
    pub(crate) use fs::{choose, read_dir, load_into_browser, RecursionType};
}

// pub(crate) use opfs::fs as opfs;

pub(crate) mod application {
    mod fs;
    //     mod web_file;
    //     mod web_fs;
    //
    //     pub use web_file::WebFile;
    //     pub use web_dir::WebDir;
    #[allow(unused_imports)]
    pub(crate) use fs::{create_file, write_all, create_dir};
}