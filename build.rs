use slint_build::compile;
fn main() {
    println!("cargo:warning================================== compile build.rs =================================\n");
    if compile("ui/login_window.slint").is_ok() {
        println!(
            "cargo:rustc-env=SLINT_INCLUDE_SUB={}/login_window.rs",
            std::env::var("OUT_DIR").unwrap()
        );
        println!("cargo:warning=compile ui/appwindow2.slint success\n");
    }
    if compile("ui/appwindow.slint").is_ok() {
        println!(
            "cargo:rustc-env=SLINT_INCLUDE_APP={}/appwindow.rs",
            std::env::var("OUT_DIR").unwrap()
        );
        println!("cargo:warning=compile ui/appwindow.slint success\n");
    }
}
