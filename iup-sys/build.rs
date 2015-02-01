extern crate "pkg-config" as pkg_config;

fn main() {
    if std::os::getenv("CARGO_FEATURE_USE_PKGCONFIG").is_some() {
      if build_pkgconfig() { return; }
      panic!("Could not find iup via pkgconfig");
    } else {
      println!("cargo:rustc-flags=-l iup");
    }
}

fn build_pkgconfig() -> bool {
    let opts = pkg_config::default_options("iup");
    pkg_config::find_library_opts("iup", &opts).is_ok()
}
