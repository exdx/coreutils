use std::process;

fn main() {
    if let Err(e) = head::get_args().and_then(head::run) {
        eprintln!("{}", e);
        process::exit(1);
    }
}
