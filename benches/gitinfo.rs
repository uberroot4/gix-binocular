use criterion::{criterion_group, criterion_main, Criterion};

use gix_test::traverse::traverse_commit_graph;

const REPO_PATH: &str = "/Volumes/RAMDisk/Binocular";

fn bench_traverse_commit_graph_t1_histogram(c: &mut Criterion) {
    let repo = gix::discover(REPO_PATH).unwrap();
    let algo = gix::diff::blob::Algorithm::Histogram;
    c.bench_function("bench_traverse_commit_graph_t1_histogram", |b| {
        b.iter(|| {
            std::hint::black_box(for _i in 1..=1 {
                let _ = traverse_commit_graph(&repo, 1, true, Some(algo));
            });
        });
    });
}

fn bench_traverse_commit_graph_t2_histogram(c: &mut Criterion) {
    let repo = gix::discover(REPO_PATH).unwrap();
    let algo = gix::diff::blob::Algorithm::Histogram;
    c.bench_function("bench_traverse_commit_graph_t2_histogram", |b| {
        b.iter(|| {
            std::hint::black_box(for _i in 1..=1 {
                let _ = traverse_commit_graph(&repo, 2, true, Some(algo));
            });
        });
    });
}
// let five_seconds = Duration::new(500, 0);
criterion_group! {
    name = benches;
    config = Criterion::default().warm_up_time(std::time::Duration::new(15,0)).measurement_time(std::time::Duration::new(60, 0)).sample_size(10);
    targets = bench_traverse_commit_graph_t1_histogram, bench_traverse_commit_graph_t2_histogram
}
criterion_main!(benches);
