#[cfg(feature = "std")]
use std::{
    string::String,
    writeln,
    println,
    boxed::Box,
    io::Write,
};

#[cfg(feature = "std")]
pub fn init_logging(file_name: Option<String>) {
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
            // println!("Verbosity turned on, no file specified");
            env_logger_build
        }
    };

    env_logger_build.init();
}



#[cfg(not(feature = "std"))]
pub fn init_logging() {}