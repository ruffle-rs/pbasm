use clap::Parser;
use pbasm::{Opt, run_main};

fn main() {
    let opt: Opt = Opt::parse();
    let exit_code = match run_main(opt) {
        Ok(()) => 0,
        Err(err) => {
            eprintln!("{err}");
            1
        }
    };

    std::process::exit(exit_code);
}
