use std::env;
fn main() {
    // https://github.com/linalg-rs/rlst/blob/main/build.rs This was helpful in understanding what to modify in build.rs
    // Note if you are on ubuntu make sure you run sudo apt-get install libblas-dev liblapack-dev to install blas and lapack libraries 
    let out_dir = env::var("OUT_DIR").unwrap();
    println!("cargo:rustc-link-search={}", out_dir.clone() + "/lib");

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();

    let mut use_system_blas_lapack = std::env::var("CARGO_FEATURE_DISABLE_SYSTEM_BLAS_LAPACK")
        .is_err()
        && std::env::var("CARGO_FEATURE_INTERNAL_BLIS").is_err();

    if target_os == "macos" && !use_system_blas_lapack {
        println!("cargo:warning=Reverting to Accelerate as BLAS/Lapack provider on Mac OS.");
        use_system_blas_lapack = true
    }

    if use_system_blas_lapack {
        if target_os == "macos" {
            println!("cargo:rustc-link-lib=framework=Accelerate");
        }

        if target_os == "linux" {
            println!("cargo:rustc-link-lib=dylib=blas");
            println!("cargo:rustc-link-lib=dylib=lapack");
        }
    }
}