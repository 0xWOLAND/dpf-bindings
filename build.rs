use std::process::Command;
use std::env;
use std::path::PathBuf;

fn main() {
    // Build using Bazel's Rust target
    let status = Command::new("bazel")
        .args(&[
            "build",
            "--verbose_failures",
            "--compilation_mode=opt",
            "//rust:dpf",
        ])
        .status()
        .expect("Failed to run bazel build");

    if !status.success() {
        panic!("Failed to build with Bazel");
    }

    // Get the Bazel output directory (bazel-bin)
    let bazel_bin = Command::new("bazel")
        .args(&["info", "bazel-bin"])
        .output()
        .expect("Failed to get bazel-bin path")
        .stdout;
    let bazel_bin = String::from_utf8(bazel_bin).unwrap().trim().to_string();

    // Tell cargo where to find the libraries
    let lib_path = PathBuf::from(&bazel_bin).join("rust");
    println!("cargo:rustc-link-search=native={}", lib_path.display());
    println!("cargo:rustc-link-lib=static=wrapper");

    let dpf_path = PathBuf::from(&bazel_bin).join("dpf");
    println!("cargo:rustc-link-search=native={}", dpf_path.display());

    // Add dependencies needed by the wrapper
    println!("cargo:rustc-link-search=native=/opt/homebrew/lib");
    
    // Core Abseil libraries
    println!("cargo:rustc-link-lib=dylib=absl_base");
    println!("cargo:rustc-link-lib=dylib=absl_int128");
    println!("cargo:rustc-link-lib=dylib=absl_strings");
    println!("cargo:rustc-link-lib=dylib=absl_throw_delegate");
    
    // Abseil containers and data structures
    println!("cargo:rustc-link-lib=dylib=absl_container");
    println!("cargo:rustc-link-lib=dylib=absl_hash");
    println!("cargo:rustc-link-lib=dylib=absl_cord");
    
    // Abseil synchronization and time
    println!("cargo:rustc-link-lib=dylib=absl_synchronization");
    println!("cargo:rustc-link-lib=dylib=absl_time");
    println!("cargo:rustc-link-lib=dylib=absl_time_zone");
    println!("cargo:rustc-link-lib=dylib=absl_civil_time");
    
    // Abseil debugging and logging
    println!("cargo:rustc-link-lib=dylib=absl_debugging_internal");
    println!("cargo:rustc-link-lib=dylib=absl_demangle_internal");
    println!("cargo:rustc-link-lib=dylib=absl_stacktrace");
    println!("cargo:rustc-link-lib=dylib=absl_symbolize");
    println!("cargo:rustc-link-lib=dylib=absl_raw_logging");
    println!("cargo:rustc-link-lib=dylib=absl_log_severity");
    
    // Add rpath to find dynamic libraries at runtime
    println!("cargo:rustc-link-arg=-Wl,-rpath,/opt/homebrew/lib");

    // Tell cargo to rebuild if any of these files change
    println!("cargo:rerun-if-changed=rust/lib.rs");
    println!("cargo:rerun-if-changed=rust/BUILD");
    println!("cargo:rerun-if-changed=dpf/wrapper.cc");
    println!("cargo:rerun-if-changed=dpf/wrapper.h");
    println!("cargo:rerun-if-changed=dpf/distributed_point_function.h");
    println!("cargo:rerun-if-changed=dpf/BUILD");
    println!("cargo:rerun-if-changed=MODULE.bazel");
    println!("cargo:rerun-if-changed=WORKSPACE.bzlmod");
} 