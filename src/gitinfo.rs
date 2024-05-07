use std::{path::Path, time::Instant};
use std::io::Write;
use gix::date::time::format;
use log::{debug, error, trace};

use diff::structs::{Args, DiffAlgorithm, OutputFormat};

fn main() {
    let args = <Args as clap::Parser>::parse_from(gix::env::args_os());
    match &args.verbose {
        Some(file) if !file.is_empty() => {
            init_logging(Some(file.to_string()));
        }
        Some(_) => {
            init_logging(None);
        }
        None => {
            println!("Verbosity not enabled");
        }
    }

    trace!("{:?}", args);
    let now = Instant::now();

    match run(args) {
        Ok(()) => {}
        Err(e) => error!("error: {e}"),
    }
    let elapsed = now.elapsed();
    debug!("Elapsed: {:.2?}", elapsed);
}

fn init_logging(file_name: Option<String>) {
    let env = env_logger::Env::default();

    let mut builder = env_logger::Builder::from_env(env);
    let mut env_logger_build = builder.format(|buf, record| {
        let style = buf.default_level_style(record.level());
        writeln!(buf, "[{} {style}{}{style:#} {} {:4} {:?}] {}",
                 chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                 record.level(),
                 if let Some(s) = record.module_path_static() { s } else { "" },
                 if let Some(v) = record.line() { v } else { 0 },
                 std::thread::current().id(),
                 record.args())
    });

    env_logger_build = match file_name {
        Some(file) => {
            println!("Verbosity level set, output to file: {}", file);
            let target = Box::new(std::fs::File::create(file).expect("Can't create file"));
            env_logger_build.target(env_logger::Target::Pipe(target))
        }
        None => {
            println!("Verbosity turned on, no file specified");
            env_logger_build
        }
    };

    env_logger_build.init();
}

fn run(args: Args) -> anyhow::Result<()> {
    let repo = gix::discover(args.git_dir.as_deref().unwrap_or(Path::new(".")))?;

    let algo = match args.algorithm {
        DiffAlgorithm::Histogram => gix::diff::blob::Algorithm::Histogram,
        DiffAlgorithm::Myers => gix::diff::blob::Algorithm::Myers,
        DiffAlgorithm::MyersMinimal => gix::diff::blob::Algorithm::MyersMinimal,
        // None => gix::diff::blob::Algorithm::Histogram,
    };

    use diff::traverse::traverse_commit_graph;
    if let Ok(result) =
        traverse_commit_graph(&repo, args.threads.unwrap_or(1), args.no_merges, Some(algo), args.breadth_first, args.committish, args.limit)
    {
        match args.output_format {
            OutputFormat::Render => {
                //println!("{:?}", result);
                let mut table = comfy_table::Table::new();

                // Setup table style
                table.load_preset(comfy_table::presets::UTF8_FULL);
                table.apply_modifier(comfy_table::modifiers::UTF8_ROUND_CORNERS);
                table.set_content_arrangement(comfy_table::ContentArrangement::Dynamic);

                // Setup table headers
                let header_color = comfy_table::Color::Green;
                let mut table_headers = vec![];
                table_headers.push(comfy_table::Cell::new("commit").fg(header_color));
                table_headers.push(comfy_table::Cell::new("parent").fg(header_color));
                table_headers.push(comfy_table::Cell::new("total_number_of_files_changed").fg(header_color));
                table_headers.push(comfy_table::Cell::new("total_number_of_insertions").fg(header_color));
                table_headers.push(comfy_table::Cell::new("total_number_of_deletions").fg(header_color));
                table_headers.push(comfy_table::Cell::new("committer_name").fg(header_color));
                table_headers.push(comfy_table::Cell::new("committer_email").fg(header_color));
                table_headers.push(comfy_table::Cell::new("commit_time").fg(header_color));
                table_headers.push(comfy_table::Cell::new("author_name").fg(header_color));
                table_headers.push(comfy_table::Cell::new("author_email").fg(header_color));
                table_headers.push(comfy_table::Cell::new("author_time").fg(header_color));
                table.set_header(table_headers);

                // Add rows to the table
                for row in result.iter() {
                    let mut table_row: Vec<comfy_table::Cell> = vec![];
                    table_row.push(comfy_table::Cell::new(row.commit.to_string()));
                    table_row.push(comfy_table::Cell::new(row.parent.map_or_else(|| "NULL".to_string(), |parent| parent.to_string())));
                    table_row.push(comfy_table::Cell::new(row.total_number_of_files_changed));
                    table_row.push(comfy_table::Cell::new(row.total_number_of_insertions));
                    table_row.push(comfy_table::Cell::new(row.total_number_of_deletions));
                    table_row.push(comfy_table::Cell::new(match &row.committer {
                        Some(c) => c.name.to_string(),
                        None => "NULL".to_string()
                    }));
                    table_row.push(comfy_table::Cell::new(match &row.committer {
                        Some(c) => c.email.to_string(),
                        None => "NULL".to_string()
                    }));
                    table_row.push(comfy_table::Cell::new(match &row.committer {
                        Some(c) => c.time.format(format::ISO8601_STRICT),
                        None => "NULL".to_string()
                    }));
                    table_row.push(comfy_table::Cell::new(match &row.author {
                        Some(c) => c.name.to_string(),
                        None => "NULL".to_string()
                    }));
                    table_row.push(comfy_table::Cell::new(match &row.author {
                        Some(c) => c.email.to_string(),
                        None => "NULL".to_string()
                    }));
                    table_row.push(comfy_table::Cell::new(match &row.author {
                        Some(c) => c.time.format(format::ISO8601_STRICT),
                        None => "NULL".to_string()
                    }));

                    table.add_row(table_row);
                }

                // Print table
                println!("{table}");
            }
            OutputFormat::CSV => {
                println!("commit,parent,total_number_of_files_changed,total_number_of_insertions,total_number_of_deletions");
                result.iter().for_each(|res| {
                    println!("{:?},{:?},{:?},{:?},{:?}", res.commit.to_string(), res.parent.unwrap().to_string(), res.total_number_of_files_changed, res.total_number_of_insertions, res.total_number_of_deletions)
                })
            }
            OutputFormat::JSON => todo!("Not yet implemented")
        }
    } else {
        error!("Failure");
    }

    Ok(())
}
