use recipe::cli::Args;

fn main() {
    if let Err(err) = Args::evaluate() {
        eprintln!("{err}")
    }
}
