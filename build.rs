use cmake::Config;
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let dst = PathBuf::from(env::var("OUT_DIR").unwrap());
    let gperf_path = env::var("TDLIB_GPERF_PATH")
        .map(PathBuf::from);

    // Output is in build since we don't run the install target
    let dst_build = dst.join("build");

    let mut cfg = Config::new("td");

    // Trim down targets to only build tdjson_static
    cfg.build_target("tdjson_static");

    // Register cargo-built dependecies
    cfg.register_dep("openssl");
    cfg.register_dep("z");

    // Above isn't enough to build on Windows
    if let Ok(path) = env::var("DEP_Z_INCLUDE") {
        cfg.define("ZLIB_INCLUDE_DIR", path);
    }

    // Specify path to gperf if specified in the environment
    if let Ok(path) = gperf_path {
        cfg.define("GPERF_EXECUTABLE:FILEPATH", path);
    }

    // Build
    cfg.build();

    // Static linking instructions
    println!("cargo:rustc-link-search=native={}", dst_build.display());
    println!("cargo:rustc-link-lib=static=tdjson_static");

    // Expose header to any dependent crates
    let dst_include = dst.join("include");
    fs::create_dir_all(&dst_include).unwrap();
    fs::copy(
        "td/td/telegram/td_json_client.h",
        dst.join("include/td_json_client.h")
    ).unwrap();
    println!("cargo:root={}", dst.to_str().unwrap());
    println!("cargo:include={}", dst_include.display());
}
