// SPDX-FileCopyrightText: Copyright (c) 2022-2025, NVIDIA CORPORATION.
// SPDX-License-Identifier: Apache-2.0
//
// Smoke tests — no GPU or libcugraph required.
// Verify enum discriminant values and struct layouts match the C headers.
// Run with: cargo test

use cugraph_sys::bindings::*;

// ── enum discriminant values ──────────────────────────────────────────────────

#[test]
fn bool_t_values() {
    assert_eq!(bool_t::FALSE as i32, 0);
    assert_eq!(bool_t::TRUE as i32, 1);
}

#[test]
fn error_code_values() {
    assert_eq!(cugraph_error_code_t::CUGRAPH_SUCCESS as i32, 0);
    assert_eq!(cugraph_error_code_t::CUGRAPH_UNKNOWN_ERROR as i32, 1);
    assert_eq!(cugraph_error_code_t::CUGRAPH_INVALID_HANDLE as i32, 2);
    assert_eq!(cugraph_error_code_t::CUGRAPH_ALLOC_ERROR as i32, 3);
    assert_eq!(cugraph_error_code_t::CUGRAPH_INVALID_INPUT as i32, 4);
    assert_eq!(cugraph_error_code_t::CUGRAPH_NOT_IMPLEMENTED as i32, 5);
    assert_eq!(cugraph_error_code_t::CUGRAPH_UNSUPPORTED_TYPE_COMBINATION as i32, 6);
}

#[test]
fn data_type_id_values() {
    assert_eq!(cugraph_data_type_id_t::INT8 as i32, 0);
    assert_eq!(cugraph_data_type_id_t::INT16 as i32, 1);
    assert_eq!(cugraph_data_type_id_t::INT32 as i32, 2);
    assert_eq!(cugraph_data_type_id_t::INT64 as i32, 3);
    assert_eq!(cugraph_data_type_id_t::UINT8 as i32, 4);
    assert_eq!(cugraph_data_type_id_t::UINT16 as i32, 5);
    assert_eq!(cugraph_data_type_id_t::UINT32 as i32, 6);
    assert_eq!(cugraph_data_type_id_t::UINT64 as i32, 7);
    assert_eq!(cugraph_data_type_id_t::FLOAT32 as i32, 8);
    assert_eq!(cugraph_data_type_id_t::FLOAT64 as i32, 9);
    assert_eq!(cugraph_data_type_id_t::SIZE_T as i32, 10);
    assert_eq!(cugraph_data_type_id_t::BOOL as i32, 11);
}

#[test]
fn k_core_degree_type_values() {
    assert_eq!(cugraph_k_core_degree_type_t::K_CORE_DEGREE_TYPE_IN as i32, 0);
    assert_eq!(cugraph_k_core_degree_type_t::K_CORE_DEGREE_TYPE_OUT as i32, 1);
    assert_eq!(cugraph_k_core_degree_type_t::K_CORE_DEGREE_TYPE_INOUT as i32, 2);
}

// ── struct layout ─────────────────────────────────────────────────────────────

#[test]
fn graph_properties_layout() {
    // Two bool_t (repr(C) i32) fields → 8 bytes, align 4
    assert_eq!(std::mem::size_of::<cugraph_graph_properties_t>(), 8);
    assert_eq!(std::mem::align_of::<cugraph_graph_properties_t>(), 4);
}

#[test]
fn graph_properties_fields() {
    let props = cugraph_graph_properties_t {
        is_symmetric: bool_t::FALSE,
        is_multigraph: bool_t::TRUE,
    };
    assert_eq!(props.is_symmetric as i32, 0);
    assert_eq!(props.is_multigraph as i32, 1);
}
