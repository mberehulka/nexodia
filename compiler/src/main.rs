use std::{time::Instant, path::{PathBuf, Path}, sync::{Arc, mpsc::{channel, Sender}, Mutex}, io::{Write, Cursor}};
use compiler::{Settings, Mesh, Asset, Image};

pub fn compile(main_path: PathBuf) {
    std::fs::create_dir_all(&main_path).unwrap();
    let (tx, rx) = channel();
    let rx = Arc::new(Mutex::new(rx));
    let mut threads = Vec::new();
    for _ in 0..num_cpus::get() {
        let rx = rx.clone();
        threads.push(std::thread::spawn(move || {
            loop {
                let rxc = rx.lock().unwrap();
                let recv = rxc.recv();
                drop(rxc);
                if let Some((path, settings)) = recv.unwrap() {
                    compile_file(path, settings)
                } else {
                    break
                }
            }
        }))
    }
    dir_loop(&main_path, &tx, Settings::default());
    for _ in 0..num_cpus::get() {
        tx.send(None).unwrap()
    }
    for thread in threads {
        thread.join().unwrap()
    }
}

fn dir_loop(dir: &Path, tx: &Sender<Option<(PathBuf, Settings)>>, mut settings: Settings) {
    settings.merge(dir);
    for path in dir.read_dir().unwrap() {
        let path = path.unwrap().path();
        if path.is_dir() {
            dir_loop(&path, tx, settings.clone())
        } else if path.is_file() {
            tx.send(Some((path.to_path_buf(), settings.clone()))).unwrap()
        }
    }
}

fn compile_file(path: PathBuf, settings: Settings) {
    let start = Instant::now();
    let bytes = match path.extension().unwrap().to_str().unwrap() {
        "gltf" | "glb" => Mesh::compile(&path, &settings).bytes(),
        "jpg" | "jpeg" | "png" => Image::compile(&path, &settings).bytes(),
        "ttf" => std::fs::read(&path).unwrap(),
        _ => return
    };
    let unc_size = size::Size::from_bytes(bytes.len());
    let bytes = zstd::encode_all(Cursor::new(bytes), settings.compression_level).unwrap();
    let comp_size = size::Size::from_bytes(bytes.len());
    std::fs::OpenOptions::new()
        .write(true)
        .read(true)
        .append(false)
        .truncate(true)
        .create(true)
        .open(crate::get_compiled_file_path(path.clone())).unwrap()
        .write_all(&bytes).unwrap();
    println!("{}\n\t{:0.3} s, {} > {}", path.display(), (Instant::now()-start).as_secs_f32(), unc_size, comp_size);
}

fn get_compiled_file_path(mut path: PathBuf) -> PathBuf {
    path.set_extension("bin");
    path
}

fn main() {
    let start = Instant::now();
    let main_path = std::path::PathBuf::from("assets/");
    compile(main_path);
    let dur = Instant::now()-start;
    println!("Rust binary finished in: {:.2} seconds", dur.as_secs_f32())
}