use std::env;
use std::process::Command;

fn main() {
    // Allows `cargo test --no-default-features` to run properly on mac os
    if env::var("CARGO_FEATURE_EXTENSION_MODULE").is_err() {
        let output = Command::new("python3")
            .args(&["-c", "import sysconfig; print(sysconfig.get_config_var('LIBDIR'))"])
            .output()
            .expect("Failed to execute python3");

        if output.status.success() {
            let lib_dir = String::from_utf8_lossy(&output.stdout).trim().to_string();
            println!("cargo:rustc-link-arg=-Wl,-rpath,{}", lib_dir);
        }
    }
}
