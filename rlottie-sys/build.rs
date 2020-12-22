extern crate bindgen;
extern crate cmake;

use std::{path::Path, process::Command};

const RLOTTIE_VERSION: &str = "0.2";
// const C_SRC: &str = "rlottie/src/binding/c/lottieanimation_capi.cpp";
const C_HEADER: &str = "rlottie/inc/rlottie_capi.h";

const BINDINGS_PATH: &str = "src/bindings.rs";

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    // git checkout `rlottie` if it doesn't exist
    if !Path::new("rlottie/.git").exists() {
        println!("debug:Cloning samsung/rlottie from github");
        let _ = Command::new("git")
            .args(&["submodule", "update", "--init"])
            .status();
    }

    println!("debug:Building samsung/rlottie v{}", RLOTTIE_VERSION);

    // build C rlottie
    let mut cmk_cfg = cmake::Config::new("rlottie");
    cmk_cfg
        .define("BUILD_SHARED_LIBS", "on")
        .define("LIB_INSTALL_DIR", ".");
    let cmk_install_path = cmk_cfg.build();
    println!(
        "debug:Installed `rlottie` to {}",
        &cmk_install_path.display()
    );
    // Command::new("cd").arg(&cmk_install_path).output().unwrap();
    // Command::new("make").args(&["-j", "2"]).output().unwrap();

    // cc::Build::new()
    //     .file(C_SRC)
    //     .includes(&[
    //         "rlottie/inc",
    //         "rlottie/src/lottie",
    //         "rlottie/src/vector",
    //         "rlottie/vs2019",
    //     ])
    //     // .static_flag(true)
    //     .compile("rlottie");

    // FIXME: hack around local <angled> include, which panics bindgen
    let mut f = std::fs::read_to_string(C_HEADER).unwrap();
    {
        println!("debug:Hacking around rlottie #include paths");
        let f = f.replace("#include <rlottiecommon.h>", "#include \"rlottiecommon.h\"");
        std::fs::write(C_HEADER, &f);
    }

    // build the bindings
    let bindings = bindgen::Builder::default()
        .rust_target(bindgen::LATEST_STABLE_RUST)
        .header(C_HEADER)
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })
        .size_t_is_usize(true)
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(BINDINGS_PATH)
        .expect("Unable to write bindings");

    // FIXME: reversing the hack
    {
        let f = f.replace("#include \"rlottiecommon.h\"", "#include <rlottiecommon.h>");
        std::fs::write(C_HEADER, &f);
    }

    // link header and lib
    let lib_path = cmk_install_path.join("build");
    println!("cargo:rustc-link-search=native={}", lib_path.display());
    println!("cargo:rustc-link-lib=dylib=rlottie");
}
