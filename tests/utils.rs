#[cfg(test)]
mod tests {
    use std::path::Path;
    use gix::bstr::BStr;
    use gix_test::git_helper::{get_trees, calculate_changes};
    const REPO_PATH: &str = "/Users/rise/Repositories/Binocular";

    #[test]
    fn test_945935697466022f2ddf7b1ea4f8cf9587b12a18() {
        let repo = gix::discover(Path::new(REPO_PATH)).unwrap();

        let (mut rewrite_cache, mut diff_cache) = get_caches(&repo);

        let child_commit = get_commit(&repo, BStr::new("945935697466022f2ddf7b1ea4f8cf9587b12a18"));

        let (current, first_parent) = get_trees(&child_commit, &repo);
        let (files_changed, insertions, deletions) =
            calculate_changes(&first_parent, &current, &mut rewrite_cache, &mut diff_cache);
        assert_eq!((files_changed, insertions, deletions), (7, 190, 0));
    }

    #[test]
    fn test_2c58aacb573d02bf41c5e4930210264db31b8b7b() {
        let repo = gix::discover(Path::new(REPO_PATH)).unwrap();

        let (mut rewrite_cache, mut diff_cache) = get_caches(&repo);

        let child_commit = get_commit(&repo, BStr::new("2c58aacb573d02bf41c5e4930210264db31b8b7b"));

        let (current, first_parent) = get_trees(&child_commit, &repo);
        let (files_changed, insertions, deletions) =
            calculate_changes(&first_parent, &current, &mut rewrite_cache, &mut diff_cache);
        assert_eq!((files_changed, insertions, deletions), (1, 10, 0));
    }

    #[test]
    fn test_fb900694931dc9de03d9dd065491290d1b814aa0() {
        let repo = gix::discover(Path::new(REPO_PATH)).unwrap();

        let (mut rewrite_cache, mut diff_cache) = get_caches(&repo);

        let child_commit = get_commit(&repo, BStr::new("fb900694931dc9de03d9dd065491290d1b814aa0"));

        let (current, first_parent) = get_trees(&child_commit, &repo);
        let (files_changed, insertions, deletions) =
            calculate_changes(&first_parent, &current, &mut rewrite_cache, &mut diff_cache);
        assert_eq!((files_changed, insertions, deletions), (4, 38, 2));
    }

    #[test]
    fn test_98ae8bc731df4e384dd41a784e4c6da01b652d7b() {
        let repo = gix::discover(Path::new(REPO_PATH)).unwrap();

        let (mut rewrite_cache, mut diff_cache) = get_caches(&repo);

        let child_commit = get_commit(&repo, BStr::new("98ae8bc731df4e384dd41a784e4c6da01b652d7b"));

        let (current, first_parent) = get_trees(&child_commit, &repo);
        let (files_changed, insertions, deletions) =
            calculate_changes(&first_parent, &current, &mut rewrite_cache, &mut diff_cache);
        assert_eq!((files_changed, insertions, deletions), (1, 1, 3));
    }

    #[test]
    fn test_728072845cd43ac0da4ece3039e2562fad130876() {
        let repo = gix::discover(Path::new(REPO_PATH)).unwrap();

        let (mut rewrite_cache, mut diff_cache) = get_caches(&repo);

        let child_commit = get_commit(&repo, BStr::new("728072845cd43ac0da4ece3039e2562fad130876"));

        let (current, first_parent) = get_trees(&child_commit, &repo);
        let (files_changed, insertions, deletions) =
            calculate_changes(&first_parent, &current, &mut rewrite_cache, &mut diff_cache);
        assert_eq!((files_changed, insertions, deletions), (2, 610, 2));
    }

    #[test]
    fn test_e2a61844ab73383919172de3bbe3b9fc80dd7f7a() {
        let repo = gix::discover(Path::new(REPO_PATH)).unwrap();

        let (mut rewrite_cache, mut diff_cache) = get_caches(&repo);

        let child_commit = get_commit(&repo, BStr::new("e2a61844ab73383919172de3bbe3b9fc80dd7f7a"));

        let (current, first_parent) = get_trees(&child_commit, &repo);
        let (files_changed, insertions, deletions) =
            calculate_changes(&first_parent, &current, &mut rewrite_cache, &mut diff_cache);
        assert_eq!((files_changed, insertions, deletions), (5, 484, 49));
    }

    #[test]
    fn test_e45e671870465b25cf4f1606c74b49ad00b8ff76() {
        let repo = gix::discover(Path::new(REPO_PATH)).unwrap();

        let (mut rewrite_cache, mut diff_cache) = get_caches(&repo);

        let child_commit = get_commit(&repo, BStr::new("e45e671870465b25cf4f1606c74b49ad00b8ff76"));

        let (current, first_parent) = get_trees(&child_commit, &repo);
        let (files_changed, insertions, deletions) =
            calculate_changes(&first_parent, &current, &mut rewrite_cache, &mut diff_cache);
        assert_eq!((files_changed, insertions, deletions), (6, 1735, 35));
    }
    #[test]
    fn test_a30fdd8247fe5265baa9f8f5ea5fcf46f3d7e905() {
        let repo = gix::discover(Path::new(REPO_PATH)).unwrap();

        let (mut rewrite_cache, mut diff_cache) = get_caches(&repo);

        let child_commit = get_commit(&repo, BStr::new("a30fdd8247fe5265baa9f8f5ea5fcf46f3d7e905"));

        let (current, first_parent) = get_trees(&child_commit, &repo);
        let (files_changed, insertions, deletions) =
            calculate_changes(&first_parent, &current, &mut rewrite_cache, &mut diff_cache);
        assert_eq!((files_changed, insertions, deletions), (1, 11, 0));
    }
    
    #[test]
    fn test_7eaf5694caed1050cdd69842f49d89e4b8ed7441() {
        let repo = gix::discover(Path::new(REPO_PATH)).unwrap();

        let (mut rewrite_cache, mut diff_cache) = get_caches(&repo);

        let child_commit = get_commit(&repo, BStr::new("7eaf5694caed1050cdd69842f49d89e4b8ed7441"));

        let (current, first_parent) = get_trees(&child_commit, &repo);
        let (files_changed, insertions, deletions) =
            calculate_changes(&first_parent, &current, &mut rewrite_cache, &mut diff_cache);
        assert_eq!((files_changed, insertions, deletions), (1, 11, 0));
    }
    
    #[test]
    fn test_bdd499c3a393b44076043b409b249f67566f4ed1() {
        let repo = gix::discover(Path::new(REPO_PATH)).unwrap();

        let (mut rewrite_cache, mut diff_cache) = get_caches(&repo);

        let child_commit = get_commit(&repo, BStr::new("bdd499c3a393b44076043b409b249f67566f4ed1"));

        let (current, first_parent) = get_trees(&child_commit, &repo);
        let (files_changed, insertions, deletions) =
            calculate_changes(&first_parent, &current, &mut rewrite_cache, &mut diff_cache);
        assert_eq!((files_changed, insertions, deletions), (9, 62, 15));
    }
    
    #[test]
    fn test_2a9510a21fc7d2853f2f5739a7c43eb6d4fa599a() {
        let repo = gix::discover(Path::new(REPO_PATH)).unwrap();

        let (mut rewrite_cache, mut diff_cache) = get_caches(&repo);

        let child_commit = get_commit(&repo, BStr::new("2a9510a21fc7d2853f2f5739a7c43eb6d4fa599a"));

        let (current, first_parent) = get_trees(&child_commit, &repo);
        let (files_changed, insertions, deletions) =
            calculate_changes(&first_parent, &current, &mut rewrite_cache, &mut diff_cache);
        assert_eq!((files_changed, insertions, deletions), (9, 62, 15));
    }
    
    #[test]
    fn test_657c280861610a6f082c30f57966e4216c7a820a() {
        let repo = gix::discover(Path::new(REPO_PATH)).unwrap();

        let (mut rewrite_cache, mut diff_cache) = get_caches(&repo);

        let child_commit = get_commit(&repo, BStr::new("657c280861610a6f082c30f57966e4216c7a820a"));

        let (current, first_parent) = get_trees(&child_commit, &repo);
        let (files_changed, insertions, deletions) =
            calculate_changes(&first_parent, &current, &mut rewrite_cache, &mut diff_cache);
        assert_eq!((files_changed, insertions, deletions), (13, 440, 60));
    }
    
    #[test]
    fn test_e795f90982dad81007cf0e336b8dc7cd113e65f0() {
        let repo = gix::discover(Path::new(REPO_PATH)).unwrap();

        let (mut rewrite_cache, mut diff_cache) = get_caches(&repo);

        let child_commit = get_commit(&repo, BStr::new("e795f90982dad81007cf0e336b8dc7cd113e65f0"));

        let (current, first_parent) = get_trees(&child_commit, &repo);
        let (files_changed, insertions, deletions) =
            calculate_changes(&first_parent, &current, &mut rewrite_cache, &mut diff_cache);
        assert_eq!((files_changed, insertions, deletions), (1, 1, 1));
    }
    #[test]
    fn test_4969b1eb178b23b3b67c5bddf6f7ff3eb91e1d74() {
        let repo = gix::discover(Path::new(REPO_PATH)).unwrap();

        let (mut rewrite_cache, mut diff_cache) = get_caches(&repo);

        let child_commit = get_commit(&repo, BStr::new("4969b1eb178b23b3b67c5bddf6f7ff3eb91e1d74"));

        let (current, first_parent) = get_trees(&child_commit, &repo);
        let (files_changed, insertions, deletions) =
            calculate_changes(&first_parent, &current, &mut rewrite_cache, &mut diff_cache);
        assert_eq!((files_changed, insertions, deletions), (17, 960, 191));
    }
    
    #[test]
    fn test_aaee072f54fcef14e3d0fd116d53b0edd2ed1cf5() {
        let repo = gix::discover(Path::new(REPO_PATH)).unwrap();

        let (mut rewrite_cache, mut diff_cache) = get_caches(&repo);

        let child_commit = get_commit(&repo, BStr::new("aaee072f54fcef14e3d0fd116d53b0edd2ed1cf5"));

        let (current, first_parent) = get_trees(&child_commit, &repo);
        let (files_changed, insertions, deletions) =
            calculate_changes(&first_parent, &current, &mut rewrite_cache, &mut diff_cache);
        // 37 files changed, 381 insertions(+), 251 deletions(-)
        assert_eq!((files_changed, insertions, deletions), (37, 381, 251));
    }
    #[test]
    fn test_d28fab26b6c4c1b8a3dda0f70f748fd7f97e24dc() {
        let repo = gix::discover(Path::new(REPO_PATH)).unwrap();

        let (mut rewrite_cache, mut diff_cache) = get_caches(&repo);

        let child_commit = get_commit(&repo, BStr::new("d28fab26b6c4c1b8a3dda0f70f748fd7f97e24dc"));

        let (current, first_parent) = get_trees(&child_commit, &repo);
        let (files_changed, insertions, deletions) =
            calculate_changes(&first_parent, &current, &mut rewrite_cache, &mut diff_cache);
        // 23 files changed, 711 insertions(+), 477 deletions(-)
        assert_eq!((files_changed, insertions, deletions), (23, 711, 477));
    }
    
    #[test]
    fn test_99eb20e1373be2154d757320492202e4ce7b5a3f() {
        let repo = gix::discover(Path::new(REPO_PATH)).unwrap();

        let (mut rewrite_cache, mut diff_cache) = get_caches(&repo);

        let child_commit = get_commit(&repo, BStr::new("99eb20e1373be2154d757320492202e4ce7b5a3f"));

        let (current, first_parent) = get_trees(&child_commit, &repo);
        let (files_changed, insertions, deletions) =
            calculate_changes(&first_parent, &current, &mut rewrite_cache, &mut diff_cache);
        // 1 file changed, 22 insertions(+), 26 deletions(-)
        assert_eq!((files_changed, insertions, deletions), (1, 22, 26));
    }
    #[test]
    fn test_50c33ceefa4e5e3e91e5fca65d11ed179a7aa7bf() {
        let repo = gix::discover(Path::new(REPO_PATH)).unwrap();

        let (mut rewrite_cache, mut diff_cache) = get_caches(&repo);

        let child_commit = get_commit(&repo, BStr::new("50c33ceefa4e5e3e91e5fca65d11ed179a7aa7bf"));

        let (current, first_parent) = get_trees(&child_commit, &repo);
        let (files_changed, insertions, deletions) =
            calculate_changes(&first_parent, &current, &mut rewrite_cache, &mut diff_cache);
        // 15 files changed, 294 insertions(+), 147 deletions(-)
        assert_eq!((files_changed, insertions, deletions), (15, 294, 147));
    }
    #[test]
    fn test_b643e616095f1f303bb809a0227f2853aae5cc7e() {
        let repo = gix::discover(Path::new(REPO_PATH)).unwrap();

        let (mut rewrite_cache, mut diff_cache) = get_caches(&repo);

        let child_commit = get_commit(&repo, BStr::new("b643e616095f1f303bb809a0227f2853aae5cc7e"));

        let (current, first_parent) = get_trees(&child_commit, &repo);
        let (files_changed, insertions, deletions) =
            calculate_changes(&first_parent, &current, &mut rewrite_cache, &mut diff_cache);
        // 194 files changed, 3663 insertions(+), 6891 deletions(-)
        assert_eq!((files_changed, insertions, deletions), (194, 3663, 6891));
    }
    
    #[test]
    fn test_2c2a095c450e4d2b8f08851ceecef0763476c1ab() {
        let repo = gix::discover(Path::new(REPO_PATH)).unwrap();

        let (mut rewrite_cache, mut diff_cache) = get_caches(&repo);

        let child_commit = get_commit(&repo, BStr::new("2c2a095c450e4d2b8f08851ceecef0763476c1ab"));

        let (current, first_parent) = get_trees(&child_commit, &repo);
        let (files_changed, insertions, deletions) =
            calculate_changes(&first_parent, &current, &mut rewrite_cache, &mut diff_cache);
        assert_eq!((files_changed, insertions, deletions), (90, 1826, 1421));
    }

    fn get_commit<'a>(repo: &'a gix::Repository, spec: &'a BStr) -> gix::Commit<'a> {
        let child_commit = repo
            .rev_parse_single(spec)
            .unwrap()
            .object()
            .unwrap()
            .try_into_commit()
            .unwrap();
        child_commit
    }

    fn get_caches(repo: &gix::Repository) -> (gix_diff::blob::Platform, gix_diff::blob::Platform) {
        let mut rewrite_cache = repo
            .diff_resource_cache(gix::diff::blob::pipeline::Mode::ToGit, Default::default())
            .unwrap();
        rewrite_cache
            .options
            .skip_internal_diff_if_external_is_configured = false;
        rewrite_cache.options.algorithm = Some(gix::diff::blob::Algorithm::Histogram);
        let diff_cache = rewrite_cache.clone();
        (rewrite_cache, diff_cache)
    }
}
