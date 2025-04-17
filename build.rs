use std::env;
use std::path::PathBuf;

fn get_default_binja_path() -> PathBuf {
    if cfg!(target_os = "windows") {
        let program_files =
            env::var("ProgramFiles").unwrap_or_else(|_| String::from("C:\\Program Files"));
        PathBuf::from(&program_files).join("Vector35\\BinaryNinja")
    } else if cfg!(target_os = "macos") {
        PathBuf::from("/Applications/Binary Ninja.app/Contents/MacOS")
    } else {
        PathBuf::from("/opt/binaryninja")
    }
}

fn main() {
    // Allow optional linking to binja core library with NO_BINJA_LINK
    // Useful for running checks in CI without requiring a Binja installation/license
    if std::env::var_os("NO_BINJA_LINK").is_some() {
        println!("cargo:warning=NO_BINJA_LINK set. Skipping Binary Ninja Core library linking");
    } else {
        let link_path = match env::var("DEP_BINARYNINJACORE_PATH") {
            Ok(path) => PathBuf::from(path),
            Err(_) => get_default_binja_path(),
        };

        // Check if the library path exists
        if !link_path.exists() {
            panic!(
                "Binary Ninja Core library path does not exist: {}. \
                Set NO_BINJA_LINK environment variable to skip linking.",
                link_path.display()
            );
        }

        println!("cargo::rustc-link-lib=dylib=binaryninjacore");
        println!("cargo::rustc-link-search={}", link_path.to_string_lossy());

        #[cfg(not(target_os = "windows"))]
        {
            println!(
                "cargo::rustc-link-arg=-Wl,-rpath,{0},-L{0}",
                link_path.to_string_lossy()
            );
        }
        println!("cargo:rerun-if-env-changed=DEP_BINARYNINJACORE_PATH");
    }

    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR specified");
    let out_dir_path = PathBuf::from(out_dir);

    // Copy all binaries to OUT_DIR for unit tests.
    let bin_dir: PathBuf = "fixtures/bin".into();
    if let Ok(entries) = std::fs::read_dir(bin_dir) {
        for entry in entries {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() {
                let file_name = path.file_name().unwrap();
                let dest_path = out_dir_path.join(file_name);
                std::fs::copy(&path, &dest_path).expect("failed to copy binary to OUT_DIR");
            }
        }
    }
}
