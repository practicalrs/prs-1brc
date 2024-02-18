#![forbid(unsafe_code)]

fn main() {
    let result = prs_1brc::run();

    if let Err(e) = result {
        eprintln!("Error {e:?}");
    }
}
