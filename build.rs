fn main() {
    if rustversion::cfg!(stable) {
        println!("cargo::rustc-check-cfg=cfg(coverage)");
    } else {
        println!("cargo::rustc-check-cfg=cfg(coverage_nightly)");
    }
}
