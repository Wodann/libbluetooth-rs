use bindgen;
use pkg_config;

use std::env;
use std::path::{Path, PathBuf};

fn main() {
    let library = pkg_config::probe_library("bluez")
        .expect("bluez/libbluetooth-dev was not installed.");

    let include_dir = Path::new(library.include_paths.get(0).unwrap())
        .join("bluetooth");

    let bindings = bindgen::Builder::default()
        .header(include_dir.join("bluetooth.h").to_string_lossy())
        .header(include_dir.join("bnep.h").to_string_lossy())
        .header(include_dir.join("cmtp.h").to_string_lossy())
        .header(include_dir.join("hci.h").to_string_lossy())
        .header(include_dir.join("hci_lib.h").to_string_lossy())
        .header(include_dir.join("hidp.h").to_string_lossy())
        .header(include_dir.join("l2cap.h").to_string_lossy())
        .header(include_dir.join("rfcomm.h").to_string_lossy())
        .header(include_dir.join("sco.h").to_string_lossy())
        .header(include_dir.join("sdp.h").to_string_lossy())
        .header(include_dir.join("sdp_lib.h").to_string_lossy())
        .use_core()
        .ctypes_prefix("libc")
        .derive_debug(false)
        .derive_default(true)
        .generate()
        .expect("Unable to generate bindings");

    let output_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(output_dir.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
