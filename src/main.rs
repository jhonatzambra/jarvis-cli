use structopt::StructOpt;

/// CLI jarvis to support all the team with the new knowlegde
#[derive(StructOpt)]
struct Cli {
    /// The username to execute the requests
    username: String,
    /// The flow to print
    flow: String,
}

fn main() {
    let args = Cli::from_args();
    println!("Hello {}, the {} will be showed!", args.username, args.flow);
}
