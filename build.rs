use std::env;

fn main(){
    // This lets you specify the path of the Gravity library through the `GRAVITY_LIB_DIR`
    // environment variable, *or* using a dynamic linker (e.g. `ld`).
    let lib_path = env::var("GRAVITY_LIB_DIR");
    match lib_path {
        Ok(lib_path) => {
            println!("cargo:rustc-link-search=native={}", lib_path);
            println!("cargo:rustc-link-lib=dylib=gravity");
        },
        Err(e) => {
            println!("Warning: `GRAVITY_LIB_DIR` not specified (E: {:?}) - defaulting to the \
                      system's dynamic linker.", e);
            println!("cargo:rustc-link-lib=dylib=gravity");
        }
    }
}