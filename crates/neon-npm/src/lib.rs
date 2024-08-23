use csv::Writer;
use gix::Repository;
use neon::prelude::*;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

fn discover_repository(git_dir: String) -> anyhow::Result<Repository> {
    let repo = gix::discover(git_dir.clone().trim())?;
    Ok(repo)
}

fn to_csv(records: Vec<diff::metrics::GitDiffMetrics>) -> Result<String, Box<dyn std::error::Error>> {
    let mut writer = Writer::from_writer(vec![]);
    writer.write_record(["commit", "parent", "total_number_of_files_changed", "total_number_of_insertions", "total_number_of_deletions"])?;
    for res in records {
        writer.write_record([res.commit.to_string(), res.parent.map_or_else(|| "NULL".to_string(), |parent| parent.to_string()), res.total_number_of_files_changed.to_string(), res.total_number_of_insertions.to_string(), res.total_number_of_deletions.to_string()])?;
    }
    Ok(String::from_utf8(writer.into_inner()?)?)
}

fn traverse(mut cx: FunctionContext) -> JsResult<JsString> /*anyhow::Result<Repository>*/ {
    let arg1 = cx
        .argument::<JsString>(0)? // Access the first argument
        .value(&mut cx);

    let arg_threads = cx
        .argument::<JsString>(1)? // Access the first argument
        .value(&mut cx);

    let repo = discover_repository(arg1).expect("Repository not found");
    let algo = gix::diff::blob::Algorithm::Histogram;
    use diff::traverse::traverse_commit_graph;

    // if let Ok(result) =
    //     traverse_commit_graph(&repo, arg_threads.unwrap_or(1), true, Some(algo), false, None, None)
    // {
    //     if let Ok(csv) = to_csv(result) {
    //         // println!("{}", csv);
    //         return Ok(cx.string(csv.to_string()));
    //     }
    // }
    Ok(cx.string("Nope"))

    // Ok(cx.string(repo.git_dir().display().to_string()))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hello", hello)?;
    cx.export_function("traverse", traverse)?;
    Ok(())
}
