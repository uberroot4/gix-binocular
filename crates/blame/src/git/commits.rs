use gix::traverse::commit::topo::{Error, Sorting};
use gix::traverse::commit::{Info, Parents};
use log::trace;

pub(crate) fn commits_topo(
    odb_handle: &gix::odb::Handle,
    source_commit_oid: &gix::ObjectId,
    commit_graph: Option<gix::commitgraph::Graph>,
) -> Vec<Result<Info, Error>> {
    let sorting = Sorting::TopoOrder;
    let parents = Parents::All;
    let commits: Vec<_> = gix::traverse::commit::topo::Builder::from_iters(
        &odb_handle,
        [*source_commit_oid],
        //Some([target_commit.id]),
        None::<Vec<gix::ObjectId>>,
    )
    .with_commit_graph(commit_graph)
    .sorting(sorting)
    .parents(parents)
    .build()
    .unwrap()
    .collect();

    trace!(
        "Found {} commits for {} ({:?}, {})",
        commits.len(),
        source_commit_oid.to_string(),
        sorting,
        match parents {
            Parents::All => "Parents::All",
            Parents::First => "Parents::First"
        }
    );

    commits
}
