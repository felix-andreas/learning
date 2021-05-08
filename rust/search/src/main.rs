use colored::Colorize;
use structopt::StructOpt;
#[derive(StructOpt)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();
    println!(
        "Search file '{}' with pattern '{}'!",
        args.path.to_str().unwrap(),
        args.pattern
    );
    let content = std::fs::read_to_string(&args.path).expect("could not read file!");
    for (i, line) in content.lines().enumerate() {
        let indices: Vec<_> = line.match_indices(&args.pattern).collect();
        if !indices.is_empty() {
            let mut position = 0;
            print!("{:02} ", i.to_string().bold());
            for (index, pattern) in indices {
                print!("{}{}", &line[position..index], &pattern.red().bold());
                position = index + pattern.len();
            }
            println!("{}", &line[position..]);
        }
    }
}
