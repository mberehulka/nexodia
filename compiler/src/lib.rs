use std::{path::{PathBuf, Path}, sync::{mpsc::{channel, Sender}, Arc, Mutex}, io::{Write, Cursor}};

mod settings;  use settings::*;
mod gl;

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
    let bytes = match path.extension().unwrap().to_str().unwrap() {
        "gltf" | "glb" => gl::compile(path.clone(), &settings),
        "json" | "bin" => {return}
        ext => return eprintln!("Extension: {ext} not supported !")
    };
    std::fs::OpenOptions::new()
        .write(true)
        .read(true)
        .append(false)
        .truncate(true)
        .create(true)
        .open(crate::get_compiled_file_path(path)).unwrap()
        .write_all(&zstd::encode_all(Cursor::new(bytes), settings.compression_level).unwrap()).unwrap();
}

fn get_compiled_file_path(mut path: PathBuf) -> PathBuf {
    path.set_extension("bin");
    path
}