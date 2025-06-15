use gix::ThreadSafeRepository;

#[derive(Debug, uniffi::Record)]
pub struct BinocularRepository {
    pub git_dir: String,
    pub work_tree: Option<String>,
    pub common_dir: Option<String>,
}

// impl BinocularRepository {
//     fn foo(&self) {
//         println!("foo");
//     }
// }

uniffi::custom_type!(ThreadSafeRepository, BinocularRepository, {
    remote,
    lower: move |r| BinocularRepository {
        common_dir: match r.common_dir {
            Some(a) => Some(a.display().to_string()),
            None => None
        },
        work_tree: match r.work_tree {
            Some(val) => Some(val.display().to_string()),
            None => None
        },
        git_dir: r.refs.git_dir().display().to_string()
    },
    try_lift: |r| Ok(gix::ThreadSafeRepository::discover(r.git_dir)?),
});
