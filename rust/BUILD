load("@rules_rust//rust:defs.bzl", "rust_library", "rust_test")

rust_library(
    name = "dpf",
    srcs = ["lib.rs"],
    deps = [
        "//dpf:wrapper",  
    ],
    rustc_flags = [
        "-C", "link-arg=-Wl,-force_load,$(location //dpf:wrapper)",
    ],
    testonly = True,
    visibility = ["//visibility:public"],
)

rust_test(
    name = "dpf_test",
    crate = ":dpf",
    deps = [
        "//dpf:wrapper", 
    ],
    rustc_flags = [
        "-C", "link-arg=-Wl,-force_load,$(location //dpf:wrapper)",
    ],
) 