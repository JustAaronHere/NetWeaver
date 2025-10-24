use std::env;
use std::path::PathBuf;

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    
    println!("cargo:rerun-if-changed=c_core/src/");
    println!("cargo:rerun-if-changed=c_core/include/");

    let mut build = cc::Build::new();
    build
        .file("c_core/src/packet_core.c")
        .file("c_core/src/network_io.c")
        .file("c_core/src/packet_parser.c")
        .file("c_core/src/raw_socket.c")
        .include("c_core/include")
        .warnings(true)
        .flag("-O3");

    match target_os.as_str() {
        "linux" => {
            build.flag("-DLINUX");
        }
        "macos" => {
            build.flag("-DMACOS");
        }
        "windows" => {
            build.flag("-DWINDOWS");
            println!("cargo:rustc-link-lib=ws2_32");
            println!("cargo:rustc-link-lib=iphlpapi");
        }
        _ => {}
    }

    build.compile("netweaver_core");

    let bindings = bindgen::Builder::default()
        .header("c_core/include/netweaver_core.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
