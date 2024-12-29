fn main() {
    // Compile our wrapper using Homebrew's abseil
    cc::Build::new()
        .cpp(true)
        .file("../../dpf/wrapper.cc")
        .include("../../")
        .include("../../bazel-bin")  // Include generated files from bazel
        .include("/opt/homebrew/include")  // Homebrew include path for ARM Macs
        .flag_if_supported("-std=c++17")
        .flag("-L/opt/homebrew/lib")  // Homebrew lib path for ARM Macs
        .flag("-labsl_container")
        .flag("-labsl_hash")
        .flag("-labsl_numeric")
        .compile("dpf_wrapper");
    
    // Tell cargo to link against abseil
    println!("cargo:rustc-link-search=/opt/homebrew/lib");
    println!("cargo:rustc-link-lib=absl_container");
    println!("cargo:rustc-link-lib=absl_hash");
    println!("cargo:rustc-link-lib=absl_numeric");
    
    println!("cargo:rerun-if-changed=../../dpf/wrapper.cc");
    println!("cargo:rerun-if-changed=../../dpf/wrapper.h");
    println!("cargo:rerun-if-changed=../../dpf/distributed_point_function.h");
    println!("cargo:rerun-if-changed=../../bazel-bin/dpf/distributed_point_function.pb.h");
} 