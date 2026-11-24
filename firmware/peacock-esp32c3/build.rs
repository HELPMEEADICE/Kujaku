use std::{env, fs, path::PathBuf};

fn main() {
    println!("cargo:rustc-link-arg=-Tlinkall.x");
    println!("cargo:rerun-if-changed=build.rs");

    let gamma: [u16; 257] =
        std::array::from_fn(|i| ((i as f64 / 256.0).powf(2.2) * 4095.0).round() as u16);
    let out = PathBuf::from(env::var_os("OUT_DIR").expect("Cargo OUT_DIR"));
    fs::write(out.join("gamma.rs"), format!("{gamma:?}")).expect("write gamma lookup table");
}
