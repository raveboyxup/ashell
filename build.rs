fn main() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    if target_os == "windows" {
        // Use windres (MinGW) when cross-compiling, or winres when building natively on Windows
        let rc_file = std::path::Path::new("ashell.rc");
        if rc_file.exists() {
            let out_dir = std::env::var("OUT_DIR").unwrap();
            let res_obj = format!("{}/ashell_res.o", out_dir);
            let status = std::process::Command::new("x86_64-w64-mingw32-windres")
                .args([
                    rc_file.to_str().unwrap(),
                    "-O",
                    "coff",
                    "-o",
                    &res_obj,
                ])
                .status();
            if let Ok(status) = status {
                if status.success() {
                    println!("cargo:rustc-link-arg={}", res_obj);
                }
            }
        }
    }
}
