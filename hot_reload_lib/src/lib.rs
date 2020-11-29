extern crate libloading as lib;
extern crate notify;

use std::path::{Path, PathBuf};
use std::{fs, thread, time};

use notify::{Event, EventKind, RecursiveMode, Watcher};
use time::Duration;

pub struct HotReloadLib {
    original_lib_path_string: String,
    original_lib_path: PathBuf,
    loaded_path: PathBuf,
    library: Option<lib::Library>,
    _watcher: notify::RecommendedWatcher,
}

impl HotReloadLib {
    pub fn new(
        folder: &str,
        lib_name: &str,
        mut on_reload: impl Fn() + Send + Sync + 'static,
    ) -> Self {
        let lib_path_string = {
            let prefix = "lib";
            let extension = "so";
            format!("{}/{}{}.{}", folder, prefix, lib_name, extension)
        };
        let lib_path = Path::new(&lib_path_string).canonicalize().unwrap();
        let (library, loaded_path) = copy_and_load_library(&lib_path);
        let original_lib_path = lib_path.clone();

        let mut watcher = notify::immediate_watcher(move |ev| {
            let ev: Event = match ev {
                Ok(ev) => ev,
                _ => return,
            };
            if ev.paths.contains(&original_lib_path) {
                on_reload();
            }
        })
        .unwrap();
        watcher.watch(folder, RecursiveMode::NonRecursive).unwrap();

        HotReloadLib {
            original_lib_path_string: lib_path_string,
            original_lib_path: lib_path,
            loaded_path,
            library: Some(library),
            _watcher: watcher,
        }
    }

    pub fn load_symbol<Signature>(&self, symbol_name: &str) -> lib::Symbol<Signature> {
        match self.library {
            Some(ref x) => unsafe {
                x.get(symbol_name.as_bytes())
                    .expect(format!("Failed to find symbol '{:?}'", symbol_name).as_str())
            },
            None => panic!(),
        }
    }

    pub fn update(&mut self) {
        self.library = None; // Work around library not reloading
        fs::remove_file(&self.loaded_path).unwrap();
        let (library, path) = copy_and_load_library(&self.original_lib_path);
        self.library = Some(library);
        self.loaded_path = path;
    }
}

impl Drop for HotReloadLib {
    fn drop(&mut self) {
        fs::remove_file(&self.loaded_path).unwrap();
    }
}

fn copy_and_load_library(lib_path: &Path) -> (lib::Library, PathBuf) {
    let unique_name = {
        let timestamp = time::SystemTime::now()
            .duration_since(time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        format!(
            "/tmp/{}-{}.so",
            lib_path.file_stem().unwrap().to_string_lossy(),
            timestamp
        )
    };
    thread::sleep(Duration::from_millis(1000));
    fs::copy(dbg!(&lib_path), &dbg!(&unique_name)).expect("Failed to copy lib to unique path");
    let unique_lib_path = Path::new(&unique_name).canonicalize().unwrap();
    (
        lib::Library::new(unique_lib_path.as_os_str())
            .expect(format!("Failed to load library '{:?}'", unique_lib_path).as_str()),
        unique_lib_path,
    )
}
