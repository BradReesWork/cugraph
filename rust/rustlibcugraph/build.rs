// SPDX-FileCopyrightText: Copyright (c) 2022-2025, NVIDIA CORPORATION.
// SPDX-License-Identifier: Apache-2.0

fn main() {
    // Allow the caller to point at a non-standard cuGraph install.
    // Falls back to /usr/local if the env var is not set.
    let cugraph_root = std::env::var("CUGRAPH_ROOT")
        .unwrap_or_else(|_| "/usr/local".to_string());

    // Tell the linker where to find libcugraph.so
    println!("cargo:rustc-link-search=native={}/lib", cugraph_root);
    println!("cargo:rustc-link-search=native={}/lib64", cugraph_root);

    // Link against libcugraph
    println!("cargo:rustc-link-lib=dylib=cugraph");

    // Re-run this script if the env var changes
    println!("cargo:rerun-if-env-changed=CUGRAPH_ROOT");
}
