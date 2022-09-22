// build.rs

extern crate cc;
use std::env;

fn main() {

    let out_dir = env::var("OUT_DIR").unwrap();
    cc::Build::new()
        .flag_if_supported("-O3")
        //.flag_if_supported("-D_GNU_SOURCE")
        .flag_if_supported("-lm")
        .flag_if_supported("-Wall")
        .file("src/hapcut2/common.c")
        .file("src/hapcut2/fragmatrix.c")
        .file("src/hapcut2/readinputbuffers.c")
        .file("src/hapcut2/pointerheap.c")
        .file("src/hapcut2/hapcut2.c")
        .file("src/hapcut2/logsum10.c")
        .compile("hapcut2");

    println!("cargo:rerun-if-changed=src/hapcut2/hapcut2.c");
    println!("cargo:rustc-link-search={}", out_dir);
    println!("cargo:rustc-link-arg-bins=-lhapcut2");

    /*cc::Build::new()
        .cpp(true)
        .shared_flag(false)
        .static_flag(true)
        .flag_if_supported("-O3")
        .flag_if_supported("-D_GNU_SOURCE")
        .flag_if_supported("-Wall")
        .flag_if_supported("-std=c++11")
        .flag_if_supported("-Isrc/poa/spoa/include/")
        .flag_if_supported("-Lsrc/poa/spoa/build/lib/")
        .flag_if_supported("-lspoa")
        .file("src/poa/poa_func.cpp")
        .compile("poa_func");

        println!("cargo:rustc-flags=-L src/poa/spoa/build/lib/ -L /home/pedge/anaconda3/envs/tscc/lib -l spoa");
    */
    // println!("cargo:rustc-flags=-L /home/pedge/anaconda3/envs/tscc/lib");
}
