# Rust Tutorial on CBlas

I'm a python developer learning Rust. In Python we have numpy and scipy. Numpy can do some linear algebra math and handle multi-dimensional arrays. However when it comes to Rust, there's ndarray which is just the array class; it doesn't contain any linear algebra functions like matrix multiplication. For that we use cblas. 


To get started with this example. 

> [!NOTE]
>  Linux **`sudo apt-get install libblas-dev liblapack-dev`**

> [!NOTE]
>  Mac **`brew install openblas lapack`**

Adding cblas `cargo add cblas`

Linking will generally fail which is why the build.rs has to be modified to point to these libraries. Example came from looking at the build.rs from https://github.com/linalg-rs/rlst/blob/main/build.rs 


```rust [build.rs]
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
```
