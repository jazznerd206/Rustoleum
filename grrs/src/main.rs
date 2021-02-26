// use structopt::StructOpt;
// use std::io::BufReader;

/// Search for a pattern in a file and display the lines that contain it.
// #[derive(StructOpt)]
// struct Cli {
//     /// The pattern and pattern type to look for
//     pattern: String,
//     /// The path to the file to read
//     #[structopt(parse(from_os_str))]
//     path: std::path::PathBuf,
// }

// fn main() {
//     let args = Cli::from_args();
//     let content = std::fs::read_to_string(&args.path)
//         .expect("could not read file");
//     for line in content.lines() {
//         if line.contains(&args.pattern) {
//             println!("{}", line);
//         }
//     }
// }


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let result = std::fs::read_to_string("test.txt");
    let content = match result {
        Ok(content) => { content },
        Err(error) => { return Err(error.into()); }
    };
    println!("file content: {}", content);
    Ok(())
}