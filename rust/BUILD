load("@rules_rust//rust:defs.bzl", "rust_library", "rust_test")

rust_library(
    name = "dpf",
    srcs = ["lib.rs"],
    testonly = True,
    deps = [
        "//dpf:wrapper",
    ],
    visibility = ["//visibility:public"],
)

rust_test(
    name = "dpf_test",
    crate = ":dpf",
    deps = [
        "//dpf:wrapper",
    ],
    rustc_flags = [
        "-C", "link-arg=-Wl,-undefined,dynamic_lookup",
    ],
) 