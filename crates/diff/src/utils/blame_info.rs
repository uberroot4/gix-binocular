use std::ops::Range;
use gix::bstr::BString;
use gix::traverse::commit::{Info, Parents};
use gix::traverse::commit::topo::{Error, Sorting};


pub(crate) fn commits_topo(odb_handle: &gix::odb::Handle, source_commit_oid: &gix::ObjectId, commit_graph: Option<gix::commitgraph::Graph>) -> Vec<Result<Info, Error>> {
    let commits: Vec<_> = gix::traverse::commit::topo::Builder::from_iters(
        &odb_handle,
        [*source_commit_oid],
        //Some([target_commit.id]),
        None::<Vec<gix::ObjectId>>
    )
        .with_commit_graph(commit_graph)
        .sorting(Sorting::TopoOrder)
        .parents(Parents::All)
        .build()
        .unwrap()
        .collect();

    commits
}