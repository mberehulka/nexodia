use std::time::Instant;

fn main() {
    let start = Instant::now();
    let main_path = std::path::PathBuf::from("assets/");
    compiler::compile(main_path);
    let dur = Instant::now()-start;
    println!("Rust binary finished in: {:.2} seconds", dur.as_secs_f32());
}