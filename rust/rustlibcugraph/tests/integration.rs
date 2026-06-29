// SPDX-FileCopyrightText: Copyright (c) 2022-2025, NVIDIA CORPORATION.
// SPDX-License-Identifier: Apache-2.0
//
// Integration tests — require a GPU and libcugraph at runtime.
// These mirror the Python tests in pylibcugraph/tests/.
//
// Run with:
//   RUN_GPU_TESTS=1 cargo test -- --include-ignored
//
// Or run a single test:
//   RUN_GPU_TESTS=1 cargo test test_sg_graph_create_free -- --include-ignored

use cugraph_sys::bindings::*;
use std::ptr;

fn gpu_tests_enabled() -> bool {
    std::env::var("RUN_GPU_TESTS").map(|v| v == "1").unwrap_or(false)
}

macro_rules! skip_without_gpu {
    () => {
        if !gpu_tests_enabled() {
            eprintln!("Skipping — set RUN_GPU_TESTS=1 to run GPU tests");
            return;
        }
    };
}

/// Panics with a descriptive message if `code` is not SUCCESS.
unsafe fn check(code: cugraph_error_code_t, error: *mut cugraph_error_t, ctx: &str) {
    if code != cugraph_error_code_t::CUGRAPH_SUCCESS {
        let msg = if !error.is_null() {
            let s = std::ffi::CStr::from_ptr(cugraph_error_message(error))
                .to_string_lossy()
                .into_owned();
            cugraph_error_free(error);
            s
        } else {
            "(no error object)".to_string()
        };
        panic!("{}: {:?} — {}", ctx, code, msg);
    }
}

/// Allocate a device array and fill it from a host slice.
unsafe fn make_i32_array(
    handle: *const cugraph_resource_handle_t,
    data: &[i32],
) -> (*mut cugraph_type_erased_device_array_t, *mut cugraph_type_erased_device_array_view_t) {
    let mut arr: *mut cugraph_type_erased_device_array_t = ptr::null_mut();
    let mut err: *mut cugraph_error_t = ptr::null_mut();
    check(
        cugraph_type_erased_device_array_create(
            handle,
            cugraph_data_type_id_t::INT32,
            data.len(),
            &mut arr,
            &mut err,
        ),
        err,
        "device_array_create i32",
    );
    let view = cugraph_type_erased_device_array_view(arr);
    let mut err2: *mut cugraph_error_t = ptr::null_mut();
    check(
        cugraph_type_erased_device_array_view_copy_from_host(
            handle,
            view,
            data.as_ptr() as *const i8,
            &mut err2,
        ),
        err2,
        "copy_from_host i32",
    );
    (arr, view)
}

/// Allocate a device array and fill it from a host f32 slice.
unsafe fn make_f32_array(
    handle: *const cugraph_resource_handle_t,
    data: &[f32],
) -> (*mut cugraph_type_erased_device_array_t, *mut cugraph_type_erased_device_array_view_t) {
    let mut arr: *mut cugraph_type_erased_device_array_t = ptr::null_mut();
    let mut err: *mut cugraph_error_t = ptr::null_mut();
    check(
        cugraph_type_erased_device_array_create(
            handle,
            cugraph_data_type_id_t::FLOAT32,
            data.len(),
            &mut arr,
            &mut err,
        ),
        err,
        "device_array_create f32",
    );
    let view = cugraph_type_erased_device_array_view(arr);
    let mut err2: *mut cugraph_error_t = ptr::null_mut();
    check(
        cugraph_type_erased_device_array_view_copy_from_host(
            handle,
            view,
            data.as_ptr() as *const i8,
            &mut err2,
        ),
        err2,
        "copy_from_host f32",
    );
    (arr, view)
}

// ── resource handle ───────────────────────────────────────────────────────────
// Mirrors: test_structure.py::test_resource_handle

#[test]
#[ignore = "requires GPU and libcugraph"]
fn test_resource_handle_create_free() {
    skip_without_gpu!();
    unsafe {
        let handle = cugraph_create_resource_handle(ptr::null_mut());
        assert!(!handle.is_null());
        assert_eq!(cugraph_resource_handle_get_rank(handle), 0);
        cugraph_free_resource_handle(handle);
    }
}

// ── device arrays ─────────────────────────────────────────────────────────────

#[test]
#[ignore = "requires GPU and libcugraph"]
fn test_device_array_roundtrip() {
    skip_without_gpu!();
    unsafe {
        let handle = cugraph_create_resource_handle(ptr::null_mut());
        let data: Vec<i32> = vec![10, 20, 30, 40];
        let (arr, view) = make_i32_array(handle, &data);

        assert_eq!(cugraph_type_erased_device_array_view_size(view), 4);
        assert_eq!(
            cugraph_type_erased_device_array_view_type(view),
            cugraph_data_type_id_t::INT32
        );

        // Copy back to host and verify
        let mut host_buf: Vec<i32> = vec![0; 4];
        let mut err: *mut cugraph_error_t = ptr::null_mut();
        check(
            cugraph_type_erased_device_array_view_copy_to_host(
                handle,
                host_buf.as_mut_ptr() as *mut i8,
                view,
                &mut err,
            ),
            err,
            "copy_to_host",
        );
        assert_eq!(host_buf, data);

        cugraph_type_erased_device_array_free(arr);
        cugraph_free_resource_handle(handle);
    }
}

// ── single-GPU graph creation (COO) ──────────────────────────────────────────
// Mirrors: test_structure.py::test_sg_graph / conftest Simple_1
//   srcs:    [0, 1, 2]
//   dsts:    [1, 2, 3]
//   weights: [1.0, 1.0, 1.0]

#[test]
#[ignore = "requires GPU and libcugraph"]
fn test_sg_graph_create_free() {
    skip_without_gpu!();
    unsafe {
        let handle = cugraph_create_resource_handle(ptr::null_mut());

        let srcs: Vec<i32> = vec![0, 1, 2];
        let dsts: Vec<i32> = vec![1, 2, 3];
        let wgts: Vec<f32> = vec![1.0, 1.0, 1.0];

        let (src_arr, src_view) = make_i32_array(handle, &srcs);
        let (dst_arr, dst_view) = make_i32_array(handle, &dsts);
        let (wgt_arr, wgt_view) = make_f32_array(handle, &wgts);

        let props = cugraph_graph_properties_t {
            is_symmetric: bool_t::FALSE,
            is_multigraph: bool_t::FALSE,
        };
        let mut graph: *mut cugraph_graph_t = ptr::null_mut();
        let mut err: *mut cugraph_error_t = ptr::null_mut();

        check(
            cugraph_graph_create_sg(
                handle,
                &props,
                ptr::null(),    // vertices — null means derive from edges
                src_view,
                dst_view,
                wgt_view,
                ptr::null(),    // edge_ids
                ptr::null(),    // edge_types
                bool_t::FALSE,  // store_transposed
                bool_t::FALSE,  // renumber
                bool_t::FALSE,  // drop_self_loops
                bool_t::FALSE,  // drop_multi_edges
                bool_t::FALSE,  // symmetrize
                bool_t::FALSE,  // do_expensive_check
                &mut graph,
                &mut err,
            ),
            err,
            "graph_create_sg Simple_1",
        );
        assert!(!graph.is_null());

        cugraph_graph_free(graph);
        cugraph_type_erased_device_array_free(src_arr);
        cugraph_type_erased_device_array_free(dst_arr);
        cugraph_type_erased_device_array_free(wgt_arr);
        cugraph_free_resource_handle(handle);
    }
}

// Mirrors: test_structure.py::test_sg_graph with invalid data (mismatched lengths)
#[test]
#[ignore = "requires GPU and libcugraph"]
fn test_sg_graph_invalid_weight_count() {
    skip_without_gpu!();
    unsafe {
        let handle = cugraph_create_resource_handle(ptr::null_mut());

        let srcs: Vec<i32> = vec![0, 1, 2];
        let dsts: Vec<i32> = vec![1, 2, 3];
        let wgts: Vec<f32> = vec![1.0, 1.0, 1.0, 1.0]; // one too many

        let (src_arr, src_view) = make_i32_array(handle, &srcs);
        let (dst_arr, dst_view) = make_i32_array(handle, &dsts);
        let (wgt_arr, wgt_view) = make_f32_array(handle, &wgts);

        let props = cugraph_graph_properties_t {
            is_symmetric: bool_t::FALSE,
            is_multigraph: bool_t::FALSE,
        };
        let mut graph: *mut cugraph_graph_t = ptr::null_mut();
        let mut err: *mut cugraph_error_t = ptr::null_mut();

        let code = cugraph_graph_create_sg(
            handle, &props, ptr::null(),
            src_view, dst_view, wgt_view,
            ptr::null(), ptr::null(),
            bool_t::FALSE, bool_t::FALSE, bool_t::FALSE,
            bool_t::FALSE, bool_t::FALSE, bool_t::TRUE, // expensive check catches it
            &mut graph, &mut err,
        );
        assert_ne!(code, cugraph_error_code_t::CUGRAPH_SUCCESS);
        if !err.is_null() { cugraph_error_free(err); }

        cugraph_type_erased_device_array_free(src_arr);
        cugraph_type_erased_device_array_free(dst_arr);
        cugraph_type_erased_device_array_free(wgt_arr);
        cugraph_free_resource_handle(handle);
    }
}

// ── RNG state ─────────────────────────────────────────────────────────────────

#[test]
#[ignore = "requires GPU and libcugraph"]
fn test_rng_state_create_free() {
    skip_without_gpu!();
    unsafe {
        let handle = cugraph_create_resource_handle(ptr::null_mut());
        let mut rng: *mut cugraph_rng_state_t = ptr::null_mut();
        let mut err: *mut cugraph_error_t = ptr::null_mut();
        check(
            cugraph_rng_state_create(handle, 42, &mut rng, &mut err),
            err,
            "rng_state_create",
        );
        assert!(!rng.is_null());
        cugraph_rng_state_free(rng);
        cugraph_free_resource_handle(handle);
    }
}

// ── error path: null handle ───────────────────────────────────────────────────
// Mirrors: test_structure.py invalid-input pytest.raises path

#[test]
#[ignore = "requires GPU and libcugraph"]
fn test_error_message_on_null_handle() {
    skip_without_gpu!();
    unsafe {
        // We need valid device array views — borrow a real handle for setup only.
        let setup_handle = cugraph_create_resource_handle(ptr::null_mut());
        let srcs: Vec<i32> = vec![0];
        let dsts: Vec<i32> = vec![1];
        let wgts: Vec<f32> = vec![1.0];
        let (src_arr, src_view) = make_i32_array(setup_handle, &srcs);
        let (dst_arr, dst_view) = make_i32_array(setup_handle, &dsts);
        let (wgt_arr, wgt_view) = make_f32_array(setup_handle, &wgts);
        cugraph_free_resource_handle(setup_handle);

        let props = cugraph_graph_properties_t {
            is_symmetric: bool_t::FALSE,
            is_multigraph: bool_t::FALSE,
        };
        let mut graph: *mut cugraph_graph_t = ptr::null_mut();
        let mut err: *mut cugraph_error_t = ptr::null_mut();

        let code = cugraph_graph_create_sg(
            ptr::null(), // null handle → should fail
            &props, ptr::null(),
            src_view, dst_view, wgt_view,
            ptr::null(), ptr::null(),
            bool_t::FALSE, bool_t::FALSE, bool_t::FALSE,
            bool_t::FALSE, bool_t::FALSE, bool_t::FALSE,
            &mut graph, &mut err,
        );

        assert_ne!(code, cugraph_error_code_t::CUGRAPH_SUCCESS);
        if !err.is_null() {
            let msg_ptr = cugraph_error_message(err);
            assert!(!msg_ptr.is_null());
            cugraph_error_free(err);
        }

        cugraph_type_erased_device_array_free(src_arr);
        cugraph_type_erased_device_array_free(dst_arr);
        cugraph_type_erased_device_array_free(wgt_arr);
    }
}
