mod utils;

use std::{path::Path, time::Instant};

use clap::Parser;
use gix::date::time::format;
use log::{debug, trace};

// use cli::diff::{Args, DiffAlgorithm, OutputFormat};
use cli::cmd::{Cli, Commands};
use crate::utils::logging;

fn main() {
    let args = Cli::parse_from(gix::env::args_os());

    match &args.global_opts.verbose {
        Some(file) if !file.is_empty() => {
            logging::init_logging(Some(file.to_string()));
        }
        Some(_) => {
            logging::init_logging(None);
        }
        None => {
            println!("Verbosity not enabled");
        }
    }

    let now = Instant::now();

    match args.command {
        Commands::Diff(diff_args) => {
            trace!("{:?}", diff_args);
            // match run(args) {
            //     Ok(()) => {}
            //     Err(e) => error!("error: {e}"),
            // }
        },
        // None => {
        //     // args.re
        // }
    }

    let elapsed = now.elapsed();
    debug!("Elapsed: {:.2?}", elapsed);
}

// fn run(args: Args) -> anyhow::Result<()> {
//     let repo = gix::discover(args.git_dir.as_deref().unwrap_or(Path::new(".")))?;
//
//     let algo = match args.algorithm {
//         DiffAlgorithm::Histogram => gix::diff::blob::Algorithm::Histogram,
//         DiffAlgorithm::Myers => gix::diff::blob::Algorithm::Myers,
//         DiffAlgorithm::MyersMinimal => gix::diff::blob::Algorithm::MyersMinimal,
//         // None => gix::diff::blob::Algorithm::Histogram,
//     };
//
//     use diff::traverse::traverse_commit_graph;
//     if let Ok(result) =
//         traverse_commit_graph(&repo, args.threads.unwrap_or(1), args.no_merges, Some(algo), args.breadth_first, args.committish, args.limit)
//     {
//         match args.output_format {
//             OutputFormat::Render => {
//                 //println!("{:?}", result);
//                 let mut table = comfy_table::Table::new();
//
//                 // Setup table style
//                 table.load_preset(comfy_table::presets::UTF8_FULL);
//                 table.apply_modifier(comfy_table::modifiers::UTF8_ROUND_CORNERS);
//                 table.set_content_arrangement(comfy_table::ContentArrangement::Dynamic);
//
//                 // Setup table headers
//                 let header_color = comfy_table::Color::Green;
//                 let mut table_headers = vec![];
//                 table_headers.push(comfy_table::Cell::new("commit").fg(header_color));
//                 table_headers.push(comfy_table::Cell::new("parent").fg(header_color));
//                 table_headers.push(comfy_table::Cell::new("total_number_of_files_changed").fg(header_color));
//                 table_headers.push(comfy_table::Cell::new("total_number_of_insertions").fg(header_color));
//                 table_headers.push(comfy_table::Cell::new("total_number_of_deletions").fg(header_color));
//                 table_headers.push(comfy_table::Cell::new("committer_name").fg(header_color));
//                 table_headers.push(comfy_table::Cell::new("committer_email").fg(header_color));
//                 table_headers.push(comfy_table::Cell::new("commit_time").fg(header_color));
//                 table_headers.push(comfy_table::Cell::new("author_name").fg(header_color));
//                 table_headers.push(comfy_table::Cell::new("author_email").fg(header_color));
//                 table_headers.push(comfy_table::Cell::new("author_time").fg(header_color));
//                 table.set_header(table_headers);
//
//                 // Add rows to the table
//                 for row in result.iter() {
//                     let mut table_row: Vec<comfy_table::Cell> = vec![];
//                     table_row.push(comfy_table::Cell::new(row.commit.to_string()));
//                     table_row.push(comfy_table::Cell::new(row.parent.map_or_else(|| "NULL".to_string(), |parent| parent.to_string())));
//                     table_row.push(comfy_table::Cell::new(row.total_number_of_files_changed));
//                     table_row.push(comfy_table::Cell::new(row.total_number_of_insertions));
//                     table_row.push(comfy_table::Cell::new(row.total_number_of_deletions));
//                     table_row.push(comfy_table::Cell::new(match &row.committer {
//                         Some(c) => c.name.to_string(),
//                         None => "NULL".to_string()
//                     }));
//                     table_row.push(comfy_table::Cell::new(match &row.committer {
//                         Some(c) => c.email.to_string(),
//                         None => "NULL".to_string()
//                     }));
//                     table_row.push(comfy_table::Cell::new(match &row.committer {
//                         Some(c) => c.time.format(format::ISO8601_STRICT),
//                         None => "NULL".to_string()
//                     }));
//                     table_row.push(comfy_table::Cell::new(match &row.author {
//                         Some(c) => c.name.to_string(),
//                         None => "NULL".to_string()
//                     }));
//                     table_row.push(comfy_table::Cell::new(match &row.author {
//                         Some(c) => c.email.to_string(),
//                         None => "NULL".to_string()
//                     }));
//                     table_row.push(comfy_table::Cell::new(match &row.author {
//                         Some(c) => c.time.format(format::ISO8601_STRICT),
//                         None => "NULL".to_string()
//                     }));
//
//                     table.add_row(table_row);
//                 }
//
//                 // Print table
//                 println!("{table}");
//             }
//             OutputFormat::CSV => {
//                 println!("commit,parent,total_number_of_files_changed,total_number_of_insertions,total_number_of_deletions");
//                 result.iter().for_each(|res| {
//                     println!("{:?},{:?},{:?},{:?},{:?}", res.commit.to_string(), res.parent.unwrap().to_string(), res.total_number_of_files_changed, res.total_number_of_insertions, res.total_number_of_deletions)
//                 })
//             }
//             OutputFormat::JSON => todo!("Not yet implemented")
//         }
//     } else {
//         error!("Failure");
//     }
//
//     Ok(())
// }
