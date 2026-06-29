// SPDX-FileCopyrightText: Copyright (c) 2022-2025, NVIDIA CORPORATION.
// SPDX-License-Identifier: Apache-2.0

use std::os::raw::*;

// ============================================================================
// types.h — basic type definitions
// ============================================================================

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum bool_t {
    FALSE = 0,
    TRUE = 1,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum cugraph_data_type_id_t {
    INT8 = 0,
    INT16 = 1,
    INT32 = 2,
    INT64 = 3,
    UINT8 = 4,
    UINT16 = 5,
    UINT32 = 6,
    UINT64 = 7,
    FLOAT32 = 8,
    FLOAT64 = 9,
    SIZE_T = 10,
    BOOL = 11,
}

/// byte_t is typedef'd to int8_t in C
pub type byte_t = i8;

// ============================================================================
// error.h — error codes and error type
// ============================================================================

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum cugraph_error_code_t {
    CUGRAPH_SUCCESS = 0,
    CUGRAPH_UNKNOWN_ERROR = 1,
    CUGRAPH_INVALID_HANDLE = 2,
    CUGRAPH_ALLOC_ERROR = 3,
    CUGRAPH_INVALID_INPUT = 4,
    CUGRAPH_NOT_IMPLEMENTED = 5,
    CUGRAPH_UNSUPPORTED_TYPE_COMBINATION = 6,
}

/// Opaque error object
#[repr(C)]
pub struct cugraph_error_t {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

// ============================================================================
// resource_handle.h — resource handle
// ============================================================================

/// Opaque resource handle
#[repr(C)]
pub struct cugraph_resource_handle_t {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

// ============================================================================
// random.h — RNG state
// ============================================================================

/// Opaque RNG state
#[repr(C)]
pub struct cugraph_rng_state_t {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

// ============================================================================
// array.h — type-erased arrays
// ============================================================================

/// Opaque device array
#[repr(C)]
pub struct cugraph_type_erased_device_array_t {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

/// Opaque device array view
#[repr(C)]
pub struct cugraph_type_erased_device_array_view_t {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

/// Opaque host array
#[repr(C)]
pub struct cugraph_type_erased_host_array_t {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

/// Opaque host array view
#[repr(C)]
pub struct cugraph_type_erased_host_array_view_t {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

// ============================================================================
// properties.h — vertex and edge property types
// ============================================================================

/// Opaque vertex property
#[repr(C)]
pub struct cugraph_vertex_property_t {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

/// Opaque edge property
#[repr(C)]
pub struct cugraph_edge_property_t {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

/// Opaque vertex property view
#[repr(C)]
pub struct cugraph_vertex_property_view_t {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

/// Opaque edge property view
#[repr(C)]
pub struct cugraph_edge_property_view_t {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

// ============================================================================
// coo.h — COO graph representation
// ============================================================================

/// Opaque COO object
#[repr(C)]
pub struct cugraph_coo_t {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

/// Opaque COO list object
#[repr(C)]
pub struct cugraph_coo_list_t {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

// ============================================================================
// graph.h — graph types
// ============================================================================

/// Opaque graph object
#[repr(C)]
pub struct cugraph_graph_t {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

/// Graph properties (has actual fields)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cugraph_graph_properties_t {
    pub is_symmetric: bool_t,
    pub is_multigraph: bool_t,
}

// ============================================================================
// algorithms.h — paths, BFS, SSSP, random walks, sampling
// ============================================================================

/// Opaque paths result
#[repr(C)]
pub struct cugraph_paths_result_t {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

/// Opaque extract-paths result
#[repr(C)]
pub struct cugraph_extract_paths_result_t {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

/// Opaque random-walk result
#[repr(C)]
pub struct cugraph_random_walk_result_t {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

/// Opaque sample result
#[repr(C)]
pub struct cugraph_sample_result_t {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

/// Opaque sampling options
#[repr(C)]
pub struct cugraph_sampling_options_t {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum cugraph_prior_sources_behavior_t {
    DEFAULT = 0,
    CARRY_OVER = 1,
    EXCLUDE = 2,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum cugraph_compression_type_t {
    COO = 0,
    CSR = 1,
    CSC = 2,
    DCSR = 3,
    DCSC = 4,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum cugraph_temporal_sampling_comparison_t {
    STRICTLY_INCREASING = 0,
    MONOTONICALLY_INCREASING = 1,
    STRICTLY_DECREASING = 2,
    MONOTONICALLY_DECREASING = 3,
    LAST = 4,
}

// ============================================================================
// centrality_algorithms.h — centrality result types
// ============================================================================

/// Opaque centrality result
#[repr(C)]
pub struct cugraph_centrality_result_t {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

/// Opaque HITS result
#[repr(C)]
pub struct cugraph_hits_result_t {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

/// Opaque edge centrality result
#[repr(C)]
pub struct cugraph_edge_centrality_result_t {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

// ============================================================================
// community_algorithms.h — community detection result types
// ============================================================================

/// Opaque triangle count result
#[repr(C)]
pub struct cugraph_triangle_count_result_t {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

/// Opaque hierarchical clustering result
#[repr(C)]
pub struct cugraph_hierarchical_clustering_result_t {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

/// Opaque clustering result
#[repr(C)]
pub struct cugraph_clustering_result_t {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

// ============================================================================
// core_algorithms.h — core number / k-core result types
// ============================================================================

/// Opaque core result
#[repr(C)]
pub struct cugraph_core_result_t {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

/// Opaque k-core result
#[repr(C)]
pub struct cugraph_k_core_result_t {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum cugraph_k_core_degree_type_t {
    K_CORE_DEGREE_TYPE_IN = 0,
    K_CORE_DEGREE_TYPE_OUT = 1,
    K_CORE_DEGREE_TYPE_INOUT = 2,
}

// ============================================================================
// graph_functions.h — vertex pairs, induced subgraph, degrees, edgelist
// ============================================================================

/// Opaque vertex pairs
#[repr(C)]
pub struct cugraph_vertex_pairs_t {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

/// Opaque induced subgraph result (Deprecated)
#[repr(C)]
pub struct cugraph_induced_subgraph_result_t {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

/// Opaque degrees result
#[repr(C)]
pub struct cugraph_degrees_result_t {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

/// Opaque edgelist
#[repr(C)]
pub struct cugraph_edgelist_t {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

// ============================================================================
// graph_generators.h — generator distribution enum
// ============================================================================

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum cugraph_generator_distribution_t {
    POWER_LAW = 0,
    UNIFORM = 1,
}

// ============================================================================
// labeling_algorithms.h — labeling result type
// ============================================================================

/// Opaque labeling result
#[repr(C)]
pub struct cugraph_labeling_result_t {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

// ============================================================================
// layout_algorithms.h — layout result type
// ============================================================================

/// Opaque layout result
#[repr(C)]
pub struct cugraph_layout_result_t {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

// ============================================================================
// lookup_src_dst.h — lookup container and result
// ============================================================================

/// Opaque lookup container
#[repr(C)]
pub struct cugraph_lookup_container_t {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

/// Opaque lookup result
#[repr(C)]
pub struct cugraph_lookup_result_t {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

// ============================================================================
// similarity_algorithms.h — similarity result type
// ============================================================================

/// Opaque similarity result
#[repr(C)]
pub struct cugraph_similarity_result_t {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

// ============================================================================
// extern "C" function declarations
// ============================================================================

unsafe extern "C" {
    // ------------------------------------------------------------------------
    // error.h
    // ------------------------------------------------------------------------

    pub fn cugraph_error_message(error: *const cugraph_error_t) -> *const c_char;

    pub fn cugraph_error_free(error: *mut cugraph_error_t);

    // ------------------------------------------------------------------------
    // resource_handle.h
    // ------------------------------------------------------------------------

    pub fn cugraph_create_resource_handle(
        raft_handle: *mut c_void,
    ) -> *mut cugraph_resource_handle_t;

    pub fn cugraph_resource_handle_get_rank(handle: *const cugraph_resource_handle_t) -> i32;

    pub fn cugraph_free_resource_handle(p_handle: *mut cugraph_resource_handle_t);

    // ------------------------------------------------------------------------
    // random.h
    // ------------------------------------------------------------------------

    pub fn cugraph_rng_state_create(
        handle: *const cugraph_resource_handle_t,
        seed: usize,
        state: *mut *mut cugraph_rng_state_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    pub fn cugraph_rng_state_free(p: *mut cugraph_rng_state_t);

    // ------------------------------------------------------------------------
    // array.h
    // ------------------------------------------------------------------------

    pub fn cugraph_type_erased_device_array_create(
        handle: *const cugraph_resource_handle_t,
        dtype: cugraph_data_type_id_t,
        n_elems: usize,
        array: *mut *mut cugraph_type_erased_device_array_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    pub fn cugraph_type_erased_device_array_free(p: *mut cugraph_type_erased_device_array_t);

    pub fn cugraph_type_erased_device_array_view(
        array: *mut cugraph_type_erased_device_array_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    pub fn cugraph_type_erased_device_array_view_create(
        pointer: *mut c_void,
        n_elems: usize,
        dtype: cugraph_data_type_id_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    pub fn cugraph_type_erased_device_array_view_free(
        p: *mut cugraph_type_erased_device_array_view_t,
    );

    pub fn cugraph_type_erased_device_array_view_size(
        p: *const cugraph_type_erased_device_array_view_t,
    ) -> usize;

    pub fn cugraph_type_erased_device_array_view_type(
        p: *const cugraph_type_erased_device_array_view_t,
    ) -> cugraph_data_type_id_t;

    pub fn cugraph_type_erased_device_array_view_pointer(
        p: *const cugraph_type_erased_device_array_view_t,
    ) -> *const c_void;

    pub fn cugraph_type_erased_host_array_create(
        handle: *const cugraph_resource_handle_t,
        dtype: cugraph_data_type_id_t,
        n_elems: usize,
        array: *mut *mut cugraph_type_erased_host_array_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    pub fn cugraph_type_erased_host_array_free(p: *mut cugraph_type_erased_host_array_t);

    pub fn cugraph_type_erased_host_array_view(
        array: *mut cugraph_type_erased_host_array_t,
    ) -> *mut cugraph_type_erased_host_array_view_t;

    pub fn cugraph_type_erased_host_array_view_create(
        pointer: *mut c_void,
        n_elems: usize,
        dtype: cugraph_data_type_id_t,
    ) -> *mut cugraph_type_erased_host_array_view_t;

    pub fn cugraph_type_erased_host_array_view_free(
        p: *mut cugraph_type_erased_host_array_view_t,
    );

    pub fn cugraph_type_erased_host_array_size(
        p: *const cugraph_type_erased_host_array_t,
    ) -> usize;

    pub fn cugraph_type_erased_host_array_type(
        p: *const cugraph_type_erased_host_array_t,
    ) -> cugraph_data_type_id_t;

    pub fn cugraph_type_erased_host_array_pointer(
        p: *const cugraph_type_erased_host_array_view_t,
    ) -> *mut c_void;

    pub fn cugraph_type_erased_device_array_view_copy_from_host(
        handle: *const cugraph_resource_handle_t,
        dst: *mut cugraph_type_erased_device_array_view_t,
        h_src: *const byte_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    pub fn cugraph_type_erased_device_array_view_copy_to_host(
        handle: *const cugraph_resource_handle_t,
        h_dst: *mut byte_t,
        src: *const cugraph_type_erased_device_array_view_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    pub fn cugraph_type_erased_device_array_view_copy(
        handle: *const cugraph_resource_handle_t,
        dst: *mut cugraph_type_erased_device_array_view_t,
        src: *const cugraph_type_erased_device_array_view_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    // ------------------------------------------------------------------------
    // coo.h
    // ------------------------------------------------------------------------

    pub fn cugraph_coo_get_sources(
        coo: *mut cugraph_coo_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    pub fn cugraph_coo_get_destinations(
        coo: *mut cugraph_coo_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    pub fn cugraph_coo_get_edge_weights(
        coo: *mut cugraph_coo_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    pub fn cugraph_coo_get_edge_id(
        coo: *mut cugraph_coo_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    pub fn cugraph_coo_get_edge_type(
        coo: *mut cugraph_coo_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    pub fn cugraph_coo_list_size(coo_list: *const cugraph_coo_list_t) -> usize;

    pub fn cugraph_coo_list_element(
        coo_list: *mut cugraph_coo_list_t,
        index: usize,
    ) -> *mut cugraph_coo_t;

    pub fn cugraph_coo_free(coo: *mut cugraph_coo_t);

    pub fn cugraph_coo_list_free(coo_list: *mut cugraph_coo_list_t);

    // ------------------------------------------------------------------------
    // graph.h
    // ------------------------------------------------------------------------

    pub fn cugraph_graph_create_sg(
        handle: *const cugraph_resource_handle_t,
        properties: *const cugraph_graph_properties_t,
        vertices: *const cugraph_type_erased_device_array_view_t,
        src: *const cugraph_type_erased_device_array_view_t,
        dst: *const cugraph_type_erased_device_array_view_t,
        weights: *const cugraph_type_erased_device_array_view_t,
        edge_ids: *const cugraph_type_erased_device_array_view_t,
        edge_types: *const cugraph_type_erased_device_array_view_t,
        store_transposed: bool_t,
        renumber: bool_t,
        drop_self_loops: bool_t,
        drop_multi_edges: bool_t,
        symmetrize: bool_t,
        check: bool_t,
        graph: *mut *mut cugraph_graph_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    pub fn cugraph_graph_create_with_times_sg(
        handle: *const cugraph_resource_handle_t,
        properties: *const cugraph_graph_properties_t,
        vertices: *const cugraph_type_erased_device_array_view_t,
        src: *const cugraph_type_erased_device_array_view_t,
        dst: *const cugraph_type_erased_device_array_view_t,
        weights: *const cugraph_type_erased_device_array_view_t,
        edge_ids: *const cugraph_type_erased_device_array_view_t,
        edge_type_ids: *const cugraph_type_erased_device_array_view_t,
        edge_start_time_ids: *const cugraph_type_erased_device_array_view_t,
        edge_end_time_ids: *const cugraph_type_erased_device_array_view_t,
        store_transposed: bool_t,
        renumber: bool_t,
        drop_self_loops: bool_t,
        drop_multi_edges: bool_t,
        symmetrize: bool_t,
        do_expensive_check: bool_t,
        graph: *mut *mut cugraph_graph_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    pub fn cugraph_graph_free(graph: *mut cugraph_graph_t);

    pub fn cugraph_graph_create_sg_from_csr(
        handle: *const cugraph_resource_handle_t,
        properties: *const cugraph_graph_properties_t,
        offsets: *const cugraph_type_erased_device_array_view_t,
        indices: *const cugraph_type_erased_device_array_view_t,
        weights: *const cugraph_type_erased_device_array_view_t,
        edge_ids: *const cugraph_type_erased_device_array_view_t,
        edge_type_ids: *const cugraph_type_erased_device_array_view_t,
        store_transposed: bool_t,
        renumber: bool_t,
        symmetrize: bool_t,
        check: bool_t,
        graph: *mut *mut cugraph_graph_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    pub fn cugraph_graph_create_mg(
        handle: *const cugraph_resource_handle_t,
        properties: *const cugraph_graph_properties_t,
        vertices: *const *const cugraph_type_erased_device_array_view_t,
        src: *const *const cugraph_type_erased_device_array_view_t,
        dst: *const *const cugraph_type_erased_device_array_view_t,
        weights: *const *const cugraph_type_erased_device_array_view_t,
        edge_ids: *const *const cugraph_type_erased_device_array_view_t,
        edge_type_ids: *const *const cugraph_type_erased_device_array_view_t,
        store_transposed: bool_t,
        num_arrays: usize,
        drop_self_loops: bool_t,
        drop_multi_edges: bool_t,
        symmetrize: bool_t,
        do_expensive_check: bool_t,
        graph: *mut *mut cugraph_graph_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    pub fn cugraph_graph_create_with_times_mg(
        handle: *const cugraph_resource_handle_t,
        properties: *const cugraph_graph_properties_t,
        vertices: *const *const cugraph_type_erased_device_array_view_t,
        src: *const *const cugraph_type_erased_device_array_view_t,
        dst: *const *const cugraph_type_erased_device_array_view_t,
        weights: *const *const cugraph_type_erased_device_array_view_t,
        edge_ids: *const *const cugraph_type_erased_device_array_view_t,
        edge_type_ids: *const *const cugraph_type_erased_device_array_view_t,
        edge_start_time_ids: *const *const cugraph_type_erased_device_array_view_t,
        edge_end_time_ids: *const *const cugraph_type_erased_device_array_view_t,
        store_transposed: bool_t,
        num_arrays: usize,
        drop_self_loops: bool_t,
        drop_multi_edges: bool_t,
        symmetrize: bool_t,
        do_expensive_check: bool_t,
        graph: *mut *mut cugraph_graph_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    // ------------------------------------------------------------------------
    // algorithms.h — paths and path extraction
    // ------------------------------------------------------------------------

    pub fn cugraph_paths_result_get_vertices(
        result: *mut cugraph_paths_result_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    pub fn cugraph_paths_result_get_distances(
        result: *mut cugraph_paths_result_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    pub fn cugraph_paths_result_get_predecessors(
        result: *mut cugraph_paths_result_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    pub fn cugraph_paths_result_free(result: *mut cugraph_paths_result_t);

    pub fn cugraph_extract_paths(
        handle: *const cugraph_resource_handle_t,
        graph: *mut cugraph_graph_t,
        sources: *const cugraph_type_erased_device_array_view_t,
        paths_result: *const cugraph_paths_result_t,
        destinations: *const cugraph_type_erased_device_array_view_t,
        result: *mut *mut cugraph_extract_paths_result_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    pub fn cugraph_extract_paths_result_get_max_path_length(
        result: *mut cugraph_extract_paths_result_t,
    ) -> usize;

    pub fn cugraph_extract_paths_result_get_paths(
        result: *mut cugraph_extract_paths_result_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    pub fn cugraph_extract_paths_result_free(result: *mut cugraph_extract_paths_result_t);

    // ------------------------------------------------------------------------
    // algorithms.h — bfs
    // ------------------------------------------------------------------------

    pub fn cugraph_bfs(
        handle: *const cugraph_resource_handle_t,
        graph: *mut cugraph_graph_t,
        sources: *mut cugraph_type_erased_device_array_view_t,
        direction_optimizing: bool_t,
        depth_limit: usize,
        compute_predecessors: bool_t,
        do_expensive_check: bool_t,
        result: *mut *mut cugraph_paths_result_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    // ------------------------------------------------------------------------
    // algorithms.h — sssp
    // ------------------------------------------------------------------------

    pub fn cugraph_sssp(
        handle: *const cugraph_resource_handle_t,
        graph: *mut cugraph_graph_t,
        source: usize,
        cutoff: f64,
        compute_predecessors: bool_t,
        do_expensive_check: bool_t,
        result: *mut *mut cugraph_paths_result_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    // ------------------------------------------------------------------------
    // algorithms.h — random_walks
    // ------------------------------------------------------------------------

    pub fn cugraph_random_walk_result_get_paths(
        result: *mut cugraph_random_walk_result_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    pub fn cugraph_random_walk_result_get_weights(
        result: *mut cugraph_random_walk_result_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    pub fn cugraph_random_walk_result_get_path_sizes(
        result: *mut cugraph_random_walk_result_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    pub fn cugraph_random_walk_result_get_max_path_length(
        result: *mut cugraph_random_walk_result_t,
    ) -> usize;

    pub fn cugraph_random_walk_result_free(result: *mut cugraph_random_walk_result_t);

    // ------------------------------------------------------------------------
    // algorithms.h — sampling (cugraph_sample_result_t accessors)
    // ------------------------------------------------------------------------

    pub fn cugraph_sample_result_get_renumber_map(
        result: *const cugraph_sample_result_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    pub fn cugraph_sample_result_get_renumber_map_offsets(
        result: *const cugraph_sample_result_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    pub fn cugraph_sample_result_get_edge_renumber_map(
        result: *const cugraph_sample_result_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    pub fn cugraph_sample_result_get_edge_renumber_map_offsets(
        result: *const cugraph_sample_result_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    /// Deprecated — use cugraph_sample_result_get_majors
    pub fn cugraph_sample_result_get_sources(
        result: *const cugraph_sample_result_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    /// Deprecated — use cugraph_sample_result_get_minors
    pub fn cugraph_sample_result_get_destinations(
        result: *const cugraph_sample_result_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    pub fn cugraph_sample_result_get_majors(
        result: *const cugraph_sample_result_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    pub fn cugraph_sample_result_get_minors(
        result: *const cugraph_sample_result_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    pub fn cugraph_sample_result_get_major_offsets(
        result: *const cugraph_sample_result_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    pub fn cugraph_sample_result_get_index(
        result: *const cugraph_sample_result_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    pub fn cugraph_sample_result_get_edge_weight(
        result: *const cugraph_sample_result_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    pub fn cugraph_sample_result_get_edge_id(
        result: *const cugraph_sample_result_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    pub fn cugraph_sample_result_get_edge_type(
        result: *const cugraph_sample_result_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    pub fn cugraph_sample_result_get_edge_start_time(
        result: *const cugraph_sample_result_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    pub fn cugraph_sample_result_get_edge_end_time(
        result: *const cugraph_sample_result_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    pub fn cugraph_sample_result_get_hop(
        result: *const cugraph_sample_result_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    pub fn cugraph_sample_result_get_label_hop_offsets(
        result: *const cugraph_sample_result_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    pub fn cugraph_sample_result_get_label_type_hop_offsets(
        result: *const cugraph_sample_result_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    pub fn cugraph_sample_result_get_start_labels(
        result: *const cugraph_sample_result_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    /// Deprecated
    pub fn cugraph_sample_result_get_offsets(
        result: *const cugraph_sample_result_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    pub fn cugraph_sample_result_free(result: *const cugraph_sample_result_t);

    pub fn cugraph_test_sample_result_create(
        handle: *const cugraph_resource_handle_t,
        srcs: *const cugraph_type_erased_device_array_view_t,
        dsts: *const cugraph_type_erased_device_array_view_t,
        edge_id: *const cugraph_type_erased_device_array_view_t,
        edge_type: *const cugraph_type_erased_device_array_view_t,
        wgt: *const cugraph_type_erased_device_array_view_t,
        hop: *const cugraph_type_erased_device_array_view_t,
        label: *const cugraph_type_erased_device_array_view_t,
        result: *mut *mut cugraph_sample_result_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    // ------------------------------------------------------------------------
    // algorithms.h — sampling options
    // ------------------------------------------------------------------------

    pub fn cugraph_sampling_options_create(
        options: *mut *mut cugraph_sampling_options_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    pub fn cugraph_sampling_set_temporal_sampling_comparison(
        options: *mut cugraph_sampling_options_t,
        comparison: cugraph_temporal_sampling_comparison_t,
    );

    pub fn cugraph_sampling_set_renumber_results(
        options: *mut cugraph_sampling_options_t,
        value: bool_t,
    );

    pub fn cugraph_sampling_set_retain_seeds(
        options: *mut cugraph_sampling_options_t,
        value: bool_t,
    );

    pub fn cugraph_sampling_set_with_replacement(
        options: *mut cugraph_sampling_options_t,
        value: bool_t,
    );

    pub fn cugraph_sampling_set_return_hops(
        options: *mut cugraph_sampling_options_t,
        value: bool_t,
    );

    pub fn cugraph_sampling_set_prior_sources_behavior(
        options: *mut cugraph_sampling_options_t,
        value: cugraph_prior_sources_behavior_t,
    );

    pub fn cugraph_sampling_set_dedupe_sources(
        options: *mut cugraph_sampling_options_t,
        value: bool_t,
    );

    pub fn cugraph_sampling_set_compress_per_hop(
        options: *mut cugraph_sampling_options_t,
        value: bool_t,
    );

    pub fn cugraph_sampling_set_compression_type(
        options: *mut cugraph_sampling_options_t,
        value: cugraph_compression_type_t,
    );

    pub fn cugraph_sampling_set_disjoint_sampling(
        options: *mut cugraph_sampling_options_t,
        value: bool_t,
    );

    pub fn cugraph_sampling_options_free(options: *mut cugraph_sampling_options_t);

    // ------------------------------------------------------------------------
    // algorithms.h — uniform random walks
    // ------------------------------------------------------------------------

    pub fn cugraph_uniform_random_walks(
        handle: *const cugraph_resource_handle_t,
        rng_state: *mut cugraph_rng_state_t,
        graph: *mut cugraph_graph_t,
        start_vertices: *const cugraph_type_erased_device_array_view_t,
        max_length: usize,
        result: *mut *mut cugraph_random_walk_result_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    // ------------------------------------------------------------------------
    // algorithms.h — biased random walks
    // ------------------------------------------------------------------------

    pub fn cugraph_biased_random_walks(
        handle: *const cugraph_resource_handle_t,
        rng_state: *mut cugraph_rng_state_t,
        graph: *mut cugraph_graph_t,
        start_vertices: *const cugraph_type_erased_device_array_view_t,
        max_length: usize,
        result: *mut *mut cugraph_random_walk_result_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    // ------------------------------------------------------------------------
    // algorithms.h — node2vec random walks
    // ------------------------------------------------------------------------

    pub fn cugraph_node2vec_random_walks(
        handle: *const cugraph_resource_handle_t,
        rng_state: *mut cugraph_rng_state_t,
        graph: *mut cugraph_graph_t,
        start_vertices: *const cugraph_type_erased_device_array_view_t,
        max_length: usize,
        p: f64,
        q: f64,
        result: *mut *mut cugraph_random_walk_result_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    // ------------------------------------------------------------------------
    // centrality_algorithms.h — pagerank
    // ------------------------------------------------------------------------

    pub fn cugraph_centrality_result_get_vertices(
        result: *mut cugraph_centrality_result_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    pub fn cugraph_centrality_result_get_values(
        result: *mut cugraph_centrality_result_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    pub fn cugraph_centrality_result_get_num_iterations(
        result: *mut cugraph_centrality_result_t,
    ) -> usize;

    pub fn cugraph_centrality_result_converged(result: *mut cugraph_centrality_result_t)
        -> bool_t;

    pub fn cugraph_centrality_result_free(result: *mut cugraph_centrality_result_t);

    pub fn cugraph_pagerank(
        handle: *const cugraph_resource_handle_t,
        graph: *mut cugraph_graph_t,
        precomputed_vertex_out_weight_vertices: *const cugraph_type_erased_device_array_view_t,
        precomputed_vertex_out_weight_sums: *const cugraph_type_erased_device_array_view_t,
        initial_guess_vertices: *const cugraph_type_erased_device_array_view_t,
        initial_guess_values: *const cugraph_type_erased_device_array_view_t,
        alpha: f64,
        epsilon: f64,
        max_iterations: usize,
        do_expensive_check: bool_t,
        result: *mut *mut cugraph_centrality_result_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    pub fn cugraph_pagerank_allow_nonconvergence(
        handle: *const cugraph_resource_handle_t,
        graph: *mut cugraph_graph_t,
        precomputed_vertex_out_weight_vertices: *const cugraph_type_erased_device_array_view_t,
        precomputed_vertex_out_weight_sums: *const cugraph_type_erased_device_array_view_t,
        initial_guess_vertices: *const cugraph_type_erased_device_array_view_t,
        initial_guess_values: *const cugraph_type_erased_device_array_view_t,
        alpha: f64,
        epsilon: f64,
        max_iterations: usize,
        do_expensive_check: bool_t,
        result: *mut *mut cugraph_centrality_result_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    pub fn cugraph_personalized_pagerank(
        handle: *const cugraph_resource_handle_t,
        graph: *mut cugraph_graph_t,
        precomputed_vertex_out_weight_vertices: *const cugraph_type_erased_device_array_view_t,
        precomputed_vertex_out_weight_sums: *const cugraph_type_erased_device_array_view_t,
        initial_guess_vertices: *const cugraph_type_erased_device_array_view_t,
        initial_guess_values: *const cugraph_type_erased_device_array_view_t,
        personalization_vertices: *const cugraph_type_erased_device_array_view_t,
        personalization_values: *const cugraph_type_erased_device_array_view_t,
        alpha: f64,
        epsilon: f64,
        max_iterations: usize,
        do_expensive_check: bool_t,
        result: *mut *mut cugraph_centrality_result_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    pub fn cugraph_personalized_pagerank_allow_nonconvergence(
        handle: *const cugraph_resource_handle_t,
        graph: *mut cugraph_graph_t,
        precomputed_vertex_out_weight_vertices: *const cugraph_type_erased_device_array_view_t,
        precomputed_vertex_out_weight_sums: *const cugraph_type_erased_device_array_view_t,
        initial_guess_vertices: *const cugraph_type_erased_device_array_view_t,
        initial_guess_values: *const cugraph_type_erased_device_array_view_t,
        personalization_vertices: *const cugraph_type_erased_device_array_view_t,
        personalization_values: *const cugraph_type_erased_device_array_view_t,
        alpha: f64,
        epsilon: f64,
        max_iterations: usize,
        do_expensive_check: bool_t,
        result: *mut *mut cugraph_centrality_result_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    // ------------------------------------------------------------------------
    // centrality_algorithms.h — eigenvector centrality
    // ------------------------------------------------------------------------

    pub fn cugraph_eigenvector_centrality(
        handle: *const cugraph_resource_handle_t,
        graph: *mut cugraph_graph_t,
        epsilon: f64,
        max_iterations: usize,
        do_expensive_check: bool_t,
        result: *mut *mut cugraph_centrality_result_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    // ------------------------------------------------------------------------
    // centrality_algorithms.h — katz centrality
    // ------------------------------------------------------------------------

    pub fn cugraph_katz_centrality(
        handle: *const cugraph_resource_handle_t,
        graph: *mut cugraph_graph_t,
        betas: *const cugraph_type_erased_device_array_view_t,
        alpha: f64,
        beta: f64,
        epsilon: f64,
        max_iterations: usize,
        do_expensive_check: bool_t,
        result: *mut *mut cugraph_centrality_result_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    // ------------------------------------------------------------------------
    // centrality_algorithms.h — hits
    // ------------------------------------------------------------------------

    pub fn cugraph_hits_result_get_vertices(
        result: *mut cugraph_hits_result_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    pub fn cugraph_hits_result_get_hubs(
        result: *mut cugraph_hits_result_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    pub fn cugraph_hits_result_get_authorities(
        result: *mut cugraph_hits_result_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    pub fn cugraph_hits_result_free(result: *mut cugraph_hits_result_t);

    pub fn cugraph_hits(
        handle: *const cugraph_resource_handle_t,
        graph: *mut cugraph_graph_t,
        tol: f64,
        max_iter: usize,
        initial_hubs_guess_vertices: *const cugraph_type_erased_device_array_view_t,
        initial_hubs_guess_values: *const cugraph_type_erased_device_array_view_t,
        normalized: bool_t,
        do_expensive_check: bool_t,
        result: *mut *mut cugraph_hits_result_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    // ------------------------------------------------------------------------
    // centrality_algorithms.h — betweenness centrality
    // ------------------------------------------------------------------------

    pub fn cugraph_betweenness_centrality(
        handle: *const cugraph_resource_handle_t,
        graph: *mut cugraph_graph_t,
        vertex_list: *const cugraph_type_erased_device_array_view_t,
        normalized: bool_t,
        include_endpoints: bool_t,
        do_expensive_check: bool_t,
        result: *mut *mut cugraph_centrality_result_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    // ------------------------------------------------------------------------
    // centrality_algorithms.h — edge betweenness centrality
    // ------------------------------------------------------------------------

    pub fn cugraph_edge_centrality_result_get_src_vertices(
        result: *mut cugraph_edge_centrality_result_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    pub fn cugraph_edge_centrality_result_get_dst_vertices(
        result: *mut cugraph_edge_centrality_result_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    pub fn cugraph_edge_centrality_result_get_edge_ids(
        result: *mut cugraph_edge_centrality_result_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    pub fn cugraph_edge_centrality_result_get_values(
        result: *mut cugraph_edge_centrality_result_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    pub fn cugraph_edge_centrality_result_free(result: *mut cugraph_edge_centrality_result_t);

    pub fn cugraph_edge_betweenness_centrality(
        handle: *const cugraph_resource_handle_t,
        graph: *mut cugraph_graph_t,
        vertex_list: *const cugraph_type_erased_device_array_view_t,
        normalized: bool_t,
        do_expensive_check: bool_t,
        result: *mut *mut cugraph_edge_centrality_result_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    // ------------------------------------------------------------------------
    // community_algorithms.h — triangle_count
    // ------------------------------------------------------------------------

    pub fn cugraph_triangle_count_result_get_vertices(
        result: *mut cugraph_triangle_count_result_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    pub fn cugraph_triangle_count_result_get_counts(
        result: *mut cugraph_triangle_count_result_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    pub fn cugraph_triangle_count_result_free(result: *mut cugraph_triangle_count_result_t);

    pub fn cugraph_triangle_count(
        handle: *const cugraph_resource_handle_t,
        graph: *mut cugraph_graph_t,
        start: *const cugraph_type_erased_device_array_view_t,
        do_expensive_check: bool_t,
        result: *mut *mut cugraph_triangle_count_result_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    // ------------------------------------------------------------------------
    // community_algorithms.h — louvain
    // ------------------------------------------------------------------------

    pub fn cugraph_hierarchical_clustering_result_get_vertices(
        result: *mut cugraph_hierarchical_clustering_result_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    pub fn cugraph_hierarchical_clustering_result_get_clusters(
        result: *mut cugraph_hierarchical_clustering_result_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    pub fn cugraph_hierarchical_clustering_result_get_modularity(
        result: *mut cugraph_hierarchical_clustering_result_t,
    ) -> f64;

    pub fn cugraph_hierarchical_clustering_result_free(
        result: *mut cugraph_hierarchical_clustering_result_t,
    );

    pub fn cugraph_louvain(
        handle: *const cugraph_resource_handle_t,
        graph: *mut cugraph_graph_t,
        max_level: usize,
        threshold: f64,
        resolution: f64,
        do_expensive_check: bool_t,
        result: *mut *mut cugraph_hierarchical_clustering_result_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    // ------------------------------------------------------------------------
    // community_algorithms.h — extract_ego
    // ------------------------------------------------------------------------

    pub fn cugraph_extract_ego(
        handle: *const cugraph_resource_handle_t,
        graph: *mut cugraph_graph_t,
        source_vertices: *const cugraph_type_erased_device_array_view_t,
        radius: usize,
        do_expensive_check: bool_t,
        result: *mut *mut cugraph_induced_subgraph_result_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    // ------------------------------------------------------------------------
    // community_algorithms.h — leiden
    // ------------------------------------------------------------------------

    pub fn cugraph_leiden(
        handle: *const cugraph_resource_handle_t,
        rng_state: *mut cugraph_rng_state_t,
        graph: *mut cugraph_graph_t,
        max_level: usize,
        resolution: f64,
        theta: f64,
        do_expensive_check: bool_t,
        result: *mut *mut cugraph_hierarchical_clustering_result_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    // ------------------------------------------------------------------------
    // community_algorithms.h — ECG
    // ------------------------------------------------------------------------

    pub fn cugraph_ecg(
        handle: *const cugraph_resource_handle_t,
        rng_state: *mut cugraph_rng_state_t,
        graph: *mut cugraph_graph_t,
        min_weight: f64,
        ensemble_size: usize,
        max_level: usize,
        threshold: f64,
        resolution: f64,
        do_expensive_check: bool_t,
        result: *mut *mut cugraph_hierarchical_clustering_result_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    // ------------------------------------------------------------------------
    // community_algorithms.h — Clustering
    // ------------------------------------------------------------------------

    pub fn cugraph_clustering_result_get_vertices(
        result: *mut cugraph_clustering_result_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    pub fn cugraph_clustering_result_get_clusters(
        result: *mut cugraph_clustering_result_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    pub fn cugraph_clustering_result_free(result: *mut cugraph_clustering_result_t);

    pub fn cugraph_balanced_cut_clustering(
        handle: *const cugraph_resource_handle_t,
        rng_state: *mut cugraph_rng_state_t,
        graph: *mut cugraph_graph_t,
        n_clusters: usize,
        n_eigenvectors: usize,
        evs_tolerance: f64,
        evs_max_iterations: i32,
        k_means_tolerance: f64,
        k_means_max_iterations: i32,
        do_expensive_check: bool_t,
        result: *mut *mut cugraph_clustering_result_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    pub fn cugraph_spectral_modularity_maximization(
        handle: *const cugraph_resource_handle_t,
        rng_state: *mut cugraph_rng_state_t,
        graph: *mut cugraph_graph_t,
        n_clusters: usize,
        n_eigenvectors: usize,
        evs_tolerance: f64,
        evs_max_iterations: i32,
        k_means_tolerance: f64,
        k_means_max_iterations: i32,
        do_expensive_check: bool_t,
        result: *mut *mut cugraph_clustering_result_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    pub fn cugraph_analyze_clustering_modularity(
        handle: *const cugraph_resource_handle_t,
        graph: *mut cugraph_graph_t,
        n_clusters: usize,
        vertices: *const cugraph_type_erased_device_array_view_t,
        clusters: *const cugraph_type_erased_device_array_view_t,
        score: *mut f64,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    pub fn cugraph_analyze_clustering_edge_cut(
        handle: *const cugraph_resource_handle_t,
        graph: *mut cugraph_graph_t,
        n_clusters: usize,
        vertices: *const cugraph_type_erased_device_array_view_t,
        clusters: *const cugraph_type_erased_device_array_view_t,
        score: *mut f64,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    pub fn cugraph_analyze_clustering_ratio_cut(
        handle: *const cugraph_resource_handle_t,
        graph: *mut cugraph_graph_t,
        n_clusters: usize,
        vertices: *const cugraph_type_erased_device_array_view_t,
        clusters: *const cugraph_type_erased_device_array_view_t,
        score: *mut f64,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    // ------------------------------------------------------------------------
    // community_algorithms.h — K truss
    // ------------------------------------------------------------------------

    pub fn cugraph_k_truss_subgraph(
        handle: *const cugraph_resource_handle_t,
        graph: *mut cugraph_graph_t,
        k: usize,
        do_expensive_check: bool_t,
        result: *mut *mut cugraph_induced_subgraph_result_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    // ------------------------------------------------------------------------
    // core_algorithms.h — core number
    // ------------------------------------------------------------------------

    pub fn cugraph_core_result_get_vertices(
        result: *mut cugraph_core_result_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    pub fn cugraph_core_result_get_core_numbers(
        result: *mut cugraph_core_result_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    pub fn cugraph_core_result_free(result: *mut cugraph_core_result_t);

    pub fn cugraph_core_number(
        handle: *const cugraph_resource_handle_t,
        graph: *mut cugraph_graph_t,
        degree_type: cugraph_k_core_degree_type_t,
        do_expensive_check: bool_t,
        result: *mut *mut cugraph_core_result_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    // ------------------------------------------------------------------------
    // core_algorithms.h — k-core
    // ------------------------------------------------------------------------

    pub fn cugraph_k_core_result_get_src_vertices(
        result: *mut cugraph_k_core_result_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    pub fn cugraph_k_core_result_get_dst_vertices(
        result: *mut cugraph_k_core_result_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    pub fn cugraph_k_core_result_get_weights(
        result: *mut cugraph_k_core_result_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    pub fn cugraph_k_core_result_free(result: *mut cugraph_k_core_result_t);

    pub fn cugraph_core_result_create(
        handle: *const cugraph_resource_handle_t,
        vertices: *mut cugraph_type_erased_device_array_view_t,
        core_numbers: *mut cugraph_type_erased_device_array_view_t,
        core_result: *mut *mut cugraph_core_result_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    pub fn cugraph_k_core(
        handle: *const cugraph_resource_handle_t,
        graph: *mut cugraph_graph_t,
        k: usize,
        degree_type: cugraph_k_core_degree_type_t,
        core_result: *const cugraph_core_result_t,
        do_expensive_check: bool_t,
        result: *mut *mut cugraph_k_core_result_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    // ------------------------------------------------------------------------
    // graph_functions.h — vertex_pairs
    // ------------------------------------------------------------------------

    pub fn cugraph_vertex_pairs_get_first(
        vertex_pairs: *mut cugraph_vertex_pairs_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    pub fn cugraph_vertex_pairs_get_second(
        vertex_pairs: *mut cugraph_vertex_pairs_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    pub fn cugraph_vertex_pairs_free(vertex_pairs: *mut cugraph_vertex_pairs_t);

    pub fn cugraph_create_vertex_pairs(
        handle: *const cugraph_resource_handle_t,
        graph: *mut cugraph_graph_t,
        first: *const cugraph_type_erased_device_array_view_t,
        second: *const cugraph_type_erased_device_array_view_t,
        vertex_pairs: *mut *mut cugraph_vertex_pairs_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    pub fn cugraph_two_hop_neighbors(
        handle: *const cugraph_resource_handle_t,
        graph: *const cugraph_graph_t,
        start_vertices: *const cugraph_type_erased_device_array_view_t,
        do_expensive_check: bool_t,
        result: *mut *mut cugraph_vertex_pairs_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    pub fn cugraph_renumber_arbitrary_edgelist(
        handle: *const cugraph_resource_handle_t,
        renumber_map: *const cugraph_type_erased_host_array_view_t,
        srcs: *mut cugraph_type_erased_device_array_view_t,
        dsts: *mut cugraph_type_erased_device_array_view_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    // ------------------------------------------------------------------------
    // graph_functions.h — induced_subgraph (Deprecated)
    // ------------------------------------------------------------------------

    pub fn cugraph_induced_subgraph_get_sources(
        induced_subgraph: *mut cugraph_induced_subgraph_result_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    pub fn cugraph_induced_subgraph_get_destinations(
        induced_subgraph: *mut cugraph_induced_subgraph_result_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    pub fn cugraph_induced_subgraph_get_edge_weights(
        induced_subgraph: *mut cugraph_induced_subgraph_result_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    pub fn cugraph_induced_subgraph_get_edge_ids(
        induced_subgraph: *mut cugraph_induced_subgraph_result_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    pub fn cugraph_induced_subgraph_get_edge_type_ids(
        induced_subgraph: *mut cugraph_induced_subgraph_result_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    pub fn cugraph_induced_subgraph_get_subgraph_offsets(
        induced_subgraph: *mut cugraph_induced_subgraph_result_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    pub fn cugraph_induced_subgraph_result_free(
        induced_subgraph: *mut cugraph_induced_subgraph_result_t,
    );

    pub fn cugraph_extract_induced_subgraph(
        handle: *const cugraph_resource_handle_t,
        graph: *mut cugraph_graph_t,
        subgraph_offsets: *const cugraph_type_erased_device_array_view_t,
        subgraph_vertices: *const cugraph_type_erased_device_array_view_t,
        do_expensive_check: bool_t,
        result: *mut *mut cugraph_induced_subgraph_result_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    // ------------------------------------------------------------------------
    // graph_functions.h — allgather
    // ------------------------------------------------------------------------

    pub fn cugraph_allgather(
        handle: *const cugraph_resource_handle_t,
        src: *const cugraph_type_erased_device_array_view_t,
        dst: *const cugraph_type_erased_device_array_view_t,
        weights: *const cugraph_type_erased_device_array_view_t,
        edge_ids: *const cugraph_type_erased_device_array_view_t,
        edge_type_ids: *const cugraph_type_erased_device_array_view_t,
        result: *mut *mut cugraph_induced_subgraph_result_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    // ------------------------------------------------------------------------
    // graph_functions.h — count multi-edges
    // ------------------------------------------------------------------------

    pub fn cugraph_count_multi_edges(
        handle: *const cugraph_resource_handle_t,
        graph: *mut cugraph_graph_t,
        do_expenive_check: bool_t,
        result: *mut usize,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    // ------------------------------------------------------------------------
    // graph_functions.h — degrees
    // ------------------------------------------------------------------------

    pub fn cugraph_in_degrees(
        handle: *const cugraph_resource_handle_t,
        graph: *mut cugraph_graph_t,
        source_vertices: *const cugraph_type_erased_device_array_view_t,
        do_expensive_check: bool_t,
        result: *mut *mut cugraph_degrees_result_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    pub fn cugraph_out_degrees(
        handle: *const cugraph_resource_handle_t,
        graph: *mut cugraph_graph_t,
        source_vertices: *const cugraph_type_erased_device_array_view_t,
        do_expensive_check: bool_t,
        result: *mut *mut cugraph_degrees_result_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    pub fn cugraph_degrees(
        handle: *const cugraph_resource_handle_t,
        graph: *mut cugraph_graph_t,
        source_vertices: *const cugraph_type_erased_device_array_view_t,
        do_expensive_check: bool_t,
        result: *mut *mut cugraph_degrees_result_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    pub fn cugraph_degrees_result_get_vertices(
        degrees_result: *mut cugraph_degrees_result_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    pub fn cugraph_degrees_result_get_in_degrees(
        degrees_result: *mut cugraph_degrees_result_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    pub fn cugraph_degrees_result_get_out_degrees(
        degrees_result: *mut cugraph_degrees_result_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    pub fn cugraph_degrees_result_free(degrees_result: *mut cugraph_degrees_result_t);

    // ------------------------------------------------------------------------
    // graph_functions.h — decompress to edgelist
    // ------------------------------------------------------------------------

    pub fn cugraph_edgelist_get_sources(
        edgelist: *mut cugraph_edgelist_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    pub fn cugraph_edgelist_get_destinations(
        edgelist: *mut cugraph_edgelist_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    pub fn cugraph_edgelist_get_edge_weights(
        edgelist: *mut cugraph_edgelist_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    pub fn cugraph_edgelist_get_edge_ids(
        edgelist: *mut cugraph_edgelist_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    pub fn cugraph_edgelist_get_edge_type_ids(
        edgelist: *mut cugraph_edgelist_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    pub fn cugraph_edgelist_get_edge_offsets(
        edgelist: *mut cugraph_edgelist_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    pub fn cugraph_edgelist_free(edgelist: *mut cugraph_edgelist_t);

    pub fn cugraph_decompress_to_edgelist(
        handle: *const cugraph_resource_handle_t,
        graph: *mut cugraph_graph_t,
        do_expensive_check: bool_t,
        result: *mut *mut cugraph_edgelist_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    // ------------------------------------------------------------------------
    // graph_functions.h — has_vertex
    // ------------------------------------------------------------------------

    pub fn cugraph_has_vertex(
        handle: *const cugraph_resource_handle_t,
        graph: *const cugraph_graph_t,
        vertices: *mut cugraph_type_erased_device_array_view_t,
        do_expensive_check: bool_t,
        result: *mut *mut cugraph_type_erased_device_array_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    // ------------------------------------------------------------------------
    // graph_functions.h — extract vertex list
    // ------------------------------------------------------------------------

    pub fn cugraph_extract_vertex_list(
        handle: *const cugraph_resource_handle_t,
        graph: *mut cugraph_graph_t,
        do_expensive_check: bool_t,
        result: *mut *mut cugraph_type_erased_device_array_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    // ------------------------------------------------------------------------
    // graph_generators.h
    // ------------------------------------------------------------------------

    pub fn cugraph_generate_rmat_edgelist(
        handle: *const cugraph_resource_handle_t,
        rng_state: *mut cugraph_rng_state_t,
        scale: usize,
        num_edges: usize,
        a: f64,
        b: f64,
        c: f64,
        clip_and_flip: bool_t,
        scramble_vertex_ids: bool_t,
        result: *mut *mut cugraph_coo_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    pub fn cugraph_generate_rmat_edgelists(
        handle: *const cugraph_resource_handle_t,
        rng_state: *mut cugraph_rng_state_t,
        n_edgelists: usize,
        min_scale: usize,
        max_scale: usize,
        edge_factor: usize,
        size_distribution: cugraph_generator_distribution_t,
        edge_distribution: cugraph_generator_distribution_t,
        clip_and_flip: bool_t,
        scramble_vertex_ids: bool_t,
        result: *mut *mut cugraph_coo_list_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    pub fn cugraph_generate_edge_weights(
        handle: *const cugraph_resource_handle_t,
        rng_state: *mut cugraph_rng_state_t,
        coo: *mut cugraph_coo_t,
        dtype: cugraph_data_type_id_t,
        minimum_weight: f64,
        maximum_weight: f64,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    pub fn cugraph_generate_edge_ids(
        handle: *const cugraph_resource_handle_t,
        coo: *mut cugraph_coo_t,
        multi_gpu: bool_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    pub fn cugraph_generate_edge_types(
        handle: *const cugraph_resource_handle_t,
        rng_state: *mut cugraph_rng_state_t,
        coo: *mut cugraph_coo_t,
        min_edge_type: i32,
        max_edge_type: i32,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    // ------------------------------------------------------------------------
    // labeling_algorithms.h — weakly connected components
    // ------------------------------------------------------------------------

    pub fn cugraph_labeling_result_get_vertices(
        result: *mut cugraph_labeling_result_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    pub fn cugraph_labeling_result_get_labels(
        result: *mut cugraph_labeling_result_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    pub fn cugraph_labeling_result_free(result: *mut cugraph_labeling_result_t);

    pub fn cugraph_weakly_connected_components(
        handle: *const cugraph_resource_handle_t,
        graph: *mut cugraph_graph_t,
        do_expensive_check: bool_t,
        result: *mut *mut cugraph_labeling_result_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    // ------------------------------------------------------------------------
    // layout_algorithms.h — force_atlas_2
    // ------------------------------------------------------------------------

    pub fn cugraph_layout_result_get_vertices(
        result: *mut cugraph_layout_result_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    pub fn cugraph_layout_result_get_x_axis(
        result: *mut cugraph_layout_result_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    pub fn cugraph_layout_result_get_y_axis(
        result: *mut cugraph_layout_result_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    pub fn cugraph_layout_result_free(result: *mut cugraph_layout_result_t);

    pub fn cugraph_force_atlas2(
        handle: *const cugraph_resource_handle_t,
        rng_state: *mut cugraph_rng_state_t,
        graph: *mut cugraph_graph_t,
        max_iter: i32,
        start_vertices: *mut cugraph_type_erased_device_array_view_t,
        x_start: *mut cugraph_type_erased_device_array_view_t,
        y_start: *mut cugraph_type_erased_device_array_view_t,
        outbound_attraction_distribution: bool_t,
        lin_log_mode: bool_t,
        prevent_overlapping: bool_t,
        vertex_radius_vertices: *mut cugraph_type_erased_device_array_view_t,
        vertex_radius_values: *mut cugraph_type_erased_device_array_view_t,
        overlap_scaling_ratio: f64,
        edge_weight_influence: f64,
        jitter_tolerance: f64,
        barnes_hut_optimize: bool_t,
        barnes_hut_theta: f64,
        scaling_ratio: f64,
        strong_gravity_mode: bool_t,
        gravity: f64,
        vertex_mobility_vertices: *mut cugraph_type_erased_device_array_view_t,
        vertex_mobility_values: *mut cugraph_type_erased_device_array_view_t,
        vertex_mass_vertices: *mut cugraph_type_erased_device_array_view_t,
        vertex_mass_values: *mut cugraph_type_erased_device_array_view_t,
        verbose: bool_t,
        do_expensive_check: bool_t,
        result: *mut *mut cugraph_layout_result_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    // ------------------------------------------------------------------------
    // lookup_src_dst.h
    // ------------------------------------------------------------------------

    pub fn cugraph_build_edge_id_and_type_to_src_dst_lookup_map(
        handle: *const cugraph_resource_handle_t,
        graph: *mut cugraph_graph_t,
        lookup_container: *mut *mut cugraph_lookup_container_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    pub fn cugraph_lookup_endpoints_from_edge_ids_and_single_type(
        handle: *const cugraph_resource_handle_t,
        graph: *mut cugraph_graph_t,
        lookup_container: *const cugraph_lookup_container_t,
        edge_ids_to_lookup: *const cugraph_type_erased_device_array_view_t,
        edge_type_to_lookup: i32,
        result: *mut *mut cugraph_lookup_result_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    pub fn cugraph_lookup_endpoints_from_edge_ids_and_types(
        handle: *const cugraph_resource_handle_t,
        graph: *mut cugraph_graph_t,
        lookup_container: *const cugraph_lookup_container_t,
        edge_ids_to_lookup: *const cugraph_type_erased_device_array_view_t,
        edge_types_to_lookup: *const cugraph_type_erased_device_array_view_t,
        result: *mut *mut cugraph_lookup_result_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    pub fn cugraph_lookup_result_get_srcs(
        result: *const cugraph_lookup_result_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    pub fn cugraph_lookup_result_get_dsts(
        result: *const cugraph_lookup_result_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    pub fn cugraph_lookup_result_free(result: *mut cugraph_lookup_result_t);

    pub fn cugraph_lookup_container_free(container: *mut cugraph_lookup_container_t);

    // ------------------------------------------------------------------------
    // sampling_algorithms.h — heterogeneous uniform neighbor sampling
    // ------------------------------------------------------------------------

    pub fn cugraph_heterogeneous_uniform_neighbor_sample(
        handle: *const cugraph_resource_handle_t,
        rng_state: *mut cugraph_rng_state_t,
        graph: *mut cugraph_graph_t,
        start_vertices: *const cugraph_type_erased_device_array_view_t,
        starting_vertex_label_offsets: *const cugraph_type_erased_device_array_view_t,
        vertex_type_offsets: *const cugraph_type_erased_device_array_view_t,
        fan_out: *const cugraph_type_erased_host_array_view_t,
        num_edge_types: i32,
        options: *const cugraph_sampling_options_t,
        do_expensive_check: bool_t,
        result: *mut *mut cugraph_sample_result_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    // ------------------------------------------------------------------------
    // sampling_algorithms.h — heterogeneous biased neighbor sampling
    // ------------------------------------------------------------------------

    pub fn cugraph_heterogeneous_biased_neighbor_sample(
        handle: *const cugraph_resource_handle_t,
        rng_state: *mut cugraph_rng_state_t,
        graph: *mut cugraph_graph_t,
        edge_biases: *const cugraph_edge_property_view_t,
        start_vertices: *const cugraph_type_erased_device_array_view_t,
        starting_vertex_label_offsets: *const cugraph_type_erased_device_array_view_t,
        vertex_type_offsets: *const cugraph_type_erased_device_array_view_t,
        fan_out: *const cugraph_type_erased_host_array_view_t,
        num_edge_types: i32,
        options: *const cugraph_sampling_options_t,
        do_expensive_check: bool_t,
        result: *mut *mut cugraph_sample_result_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    // ------------------------------------------------------------------------
    // sampling_algorithms.h — homogeneous uniform neighbor sampling
    // ------------------------------------------------------------------------

    pub fn cugraph_homogeneous_uniform_neighbor_sample(
        handle: *const cugraph_resource_handle_t,
        rng_state: *mut cugraph_rng_state_t,
        graph: *mut cugraph_graph_t,
        start_vertices: *const cugraph_type_erased_device_array_view_t,
        starting_vertex_label_offsets: *const cugraph_type_erased_device_array_view_t,
        fan_out: *const cugraph_type_erased_host_array_view_t,
        options: *const cugraph_sampling_options_t,
        do_expensive_check: bool_t,
        result: *mut *mut cugraph_sample_result_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    // ------------------------------------------------------------------------
    // sampling_algorithms.h — homogeneous biased neighbor sampling
    // ------------------------------------------------------------------------

    pub fn cugraph_homogeneous_biased_neighbor_sample(
        handle: *const cugraph_resource_handle_t,
        rng_state: *mut cugraph_rng_state_t,
        graph: *mut cugraph_graph_t,
        edge_biases: *const cugraph_edge_property_view_t,
        start_vertices: *const cugraph_type_erased_device_array_view_t,
        starting_vertex_label_offsets: *const cugraph_type_erased_device_array_view_t,
        fan_out: *const cugraph_type_erased_host_array_view_t,
        options: *const cugraph_sampling_options_t,
        do_expensive_check: bool_t,
        result: *mut *mut cugraph_sample_result_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    // ------------------------------------------------------------------------
    // sampling_algorithms.h — test uniform neighborhood sample result create
    // ------------------------------------------------------------------------

    pub fn cugraph_test_uniform_neighborhood_sample_result_create(
        handle: *const cugraph_resource_handle_t,
        srcs: *const cugraph_type_erased_device_array_view_t,
        dsts: *const cugraph_type_erased_device_array_view_t,
        edge_id: *const cugraph_type_erased_device_array_view_t,
        edge_type: *const cugraph_type_erased_device_array_view_t,
        weight: *const cugraph_type_erased_device_array_view_t,
        hop: *const cugraph_type_erased_device_array_view_t,
        label: *const cugraph_type_erased_device_array_view_t,
        result: *mut *mut cugraph_sample_result_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    // ------------------------------------------------------------------------
    // sampling_algorithms.h — random vertices selection
    // ------------------------------------------------------------------------

    pub fn cugraph_select_random_vertices(
        handle: *const cugraph_resource_handle_t,
        graph: *const cugraph_graph_t,
        rng_state: *mut cugraph_rng_state_t,
        num_vertices: usize,
        vertices: *mut *mut cugraph_type_erased_device_array_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    // ------------------------------------------------------------------------
    // sampling_algorithms.h — negative sampling
    // ------------------------------------------------------------------------

    pub fn cugraph_negative_sampling(
        handle: *const cugraph_resource_handle_t,
        rng_state: *mut cugraph_rng_state_t,
        graph: *mut cugraph_graph_t,
        vertices: *const cugraph_type_erased_device_array_view_t,
        src_bias: *const cugraph_type_erased_device_array_view_t,
        dst_bias: *const cugraph_type_erased_device_array_view_t,
        num_samples: usize,
        remove_duplicates: bool_t,
        remove_false_negatives: bool_t,
        exact_number_of_samples: bool_t,
        do_expensive_check: bool_t,
        result: *mut *mut cugraph_coo_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    // ------------------------------------------------------------------------
    // sampling_algorithms.h — homogeneous uniform temporal neighbor sampling
    // ------------------------------------------------------------------------

    pub fn cugraph_homogeneous_uniform_temporal_neighbor_sample(
        handle: *const cugraph_resource_handle_t,
        rng_state: *mut cugraph_rng_state_t,
        graph: *mut cugraph_graph_t,
        temporal_property_name: *const c_char,
        start_vertices: *const cugraph_type_erased_device_array_view_t,
        starting_vertex_times: *const cugraph_type_erased_device_array_view_t,
        starting_vertex_label_offsets: *const cugraph_type_erased_device_array_view_t,
        fan_out: *const cugraph_type_erased_host_array_view_t,
        sampling_options: *const cugraph_sampling_options_t,
        do_expensive_check: bool_t,
        result: *mut *mut cugraph_sample_result_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    // ------------------------------------------------------------------------
    // sampling_algorithms.h — homogeneous biased temporal neighbor sampling
    // ------------------------------------------------------------------------

    pub fn cugraph_homogeneous_biased_temporal_neighbor_sample(
        handle: *const cugraph_resource_handle_t,
        rng_state: *mut cugraph_rng_state_t,
        graph: *mut cugraph_graph_t,
        temporal_property_name: *const c_char,
        edge_biases: *const cugraph_edge_property_view_t,
        start_vertices: *const cugraph_type_erased_device_array_view_t,
        starting_vertex_times: *const cugraph_type_erased_device_array_view_t,
        starting_vertex_label_offsets: *const cugraph_type_erased_device_array_view_t,
        fan_out: *const cugraph_type_erased_host_array_view_t,
        sampling_options: *const cugraph_sampling_options_t,
        do_expensive_check: bool_t,
        result: *mut *mut cugraph_sample_result_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    // ------------------------------------------------------------------------
    // sampling_algorithms.h — heterogeneous uniform temporal neighbor sampling
    // ------------------------------------------------------------------------

    pub fn cugraph_heterogeneous_uniform_temporal_neighbor_sample(
        handle: *const cugraph_resource_handle_t,
        rng_state: *mut cugraph_rng_state_t,
        graph: *mut cugraph_graph_t,
        temporal_property_name: *const c_char,
        start_vertices: *const cugraph_type_erased_device_array_view_t,
        starting_vertex_times: *const cugraph_type_erased_device_array_view_t,
        starting_vertex_label_offsets: *const cugraph_type_erased_device_array_view_t,
        vertex_type_offsets: *const cugraph_type_erased_device_array_view_t,
        fan_out: *const cugraph_type_erased_host_array_view_t,
        num_edge_types: i32,
        sampling_options: *const cugraph_sampling_options_t,
        do_expensive_check: bool_t,
        result: *mut *mut cugraph_sample_result_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    // ------------------------------------------------------------------------
    // sampling_algorithms.h — heterogeneous biased temporal neighbor sampling
    // ------------------------------------------------------------------------

    pub fn cugraph_heterogeneous_biased_temporal_neighbor_sample(
        handle: *const cugraph_resource_handle_t,
        rng_state: *mut cugraph_rng_state_t,
        graph: *mut cugraph_graph_t,
        temporal_property_name: *const c_char,
        edge_biases: *const cugraph_edge_property_view_t,
        start_vertices: *const cugraph_type_erased_device_array_view_t,
        starting_vertex_times: *const cugraph_type_erased_device_array_view_t,
        starting_vertex_label_offsets: *const cugraph_type_erased_device_array_view_t,
        vertex_type_offsets: *const cugraph_type_erased_device_array_view_t,
        fan_out: *const cugraph_type_erased_host_array_view_t,
        num_edge_types: i32,
        sampling_options: *const cugraph_sampling_options_t,
        do_expensive_check: bool_t,
        result: *mut *mut cugraph_sample_result_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    // ------------------------------------------------------------------------
    // similarity_algorithms.h
    // ------------------------------------------------------------------------

    pub fn cugraph_similarity_result_get_vertex_pairs(
        result: *mut cugraph_similarity_result_t,
    ) -> *mut cugraph_vertex_pairs_t;

    pub fn cugraph_similarity_result_get_similarity(
        result: *mut cugraph_similarity_result_t,
    ) -> *mut cugraph_type_erased_device_array_view_t;

    pub fn cugraph_similarity_result_free(result: *mut cugraph_similarity_result_t);

    // ------------------------------------------------------------------------
    // similarity_algorithms.h — jaccard coefficients
    // ------------------------------------------------------------------------

    pub fn cugraph_jaccard_coefficients(
        handle: *const cugraph_resource_handle_t,
        graph: *mut cugraph_graph_t,
        vertex_pairs: *const cugraph_vertex_pairs_t,
        use_weight: bool_t,
        do_expensive_check: bool_t,
        result: *mut *mut cugraph_similarity_result_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    // ------------------------------------------------------------------------
    // similarity_algorithms.h — all-pairs jaccard coefficients
    // ------------------------------------------------------------------------

    pub fn cugraph_all_pairs_jaccard_coefficients(
        handle: *const cugraph_resource_handle_t,
        graph: *mut cugraph_graph_t,
        vertices: *const cugraph_type_erased_device_array_view_t,
        use_weight: bool_t,
        topk: usize,
        do_expensive_check: bool_t,
        result: *mut *mut cugraph_similarity_result_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    // ------------------------------------------------------------------------
    // similarity_algorithms.h — sorensen coefficients
    // ------------------------------------------------------------------------

    pub fn cugraph_sorensen_coefficients(
        handle: *const cugraph_resource_handle_t,
        graph: *mut cugraph_graph_t,
        vertex_pairs: *const cugraph_vertex_pairs_t,
        use_weight: bool_t,
        do_expensive_check: bool_t,
        result: *mut *mut cugraph_similarity_result_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    // ------------------------------------------------------------------------
    // similarity_algorithms.h — all-pairs sorensen coefficients
    // ------------------------------------------------------------------------

    pub fn cugraph_all_pairs_sorensen_coefficients(
        handle: *const cugraph_resource_handle_t,
        graph: *mut cugraph_graph_t,
        vertices: *const cugraph_type_erased_device_array_view_t,
        use_weight: bool_t,
        topk: usize,
        do_expensive_check: bool_t,
        result: *mut *mut cugraph_similarity_result_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    // ------------------------------------------------------------------------
    // similarity_algorithms.h — overlap coefficients
    // ------------------------------------------------------------------------

    pub fn cugraph_overlap_coefficients(
        handle: *const cugraph_resource_handle_t,
        graph: *mut cugraph_graph_t,
        vertex_pairs: *const cugraph_vertex_pairs_t,
        use_weight: bool_t,
        do_expensive_check: bool_t,
        result: *mut *mut cugraph_similarity_result_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    // ------------------------------------------------------------------------
    // similarity_algorithms.h — all-pairs overlap coefficients
    // ------------------------------------------------------------------------

    pub fn cugraph_all_pairs_overlap_coefficients(
        handle: *const cugraph_resource_handle_t,
        graph: *mut cugraph_graph_t,
        vertices: *const cugraph_type_erased_device_array_view_t,
        use_weight: bool_t,
        topk: usize,
        do_expensive_check: bool_t,
        result: *mut *mut cugraph_similarity_result_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    // ------------------------------------------------------------------------
    // similarity_algorithms.h — cosine coefficients
    // ------------------------------------------------------------------------

    pub fn cugraph_cosine_similarity_coefficients(
        handle: *const cugraph_resource_handle_t,
        graph: *mut cugraph_graph_t,
        vertex_pairs: *const cugraph_vertex_pairs_t,
        use_weight: bool_t,
        do_expensive_check: bool_t,
        result: *mut *mut cugraph_similarity_result_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    // ------------------------------------------------------------------------
    // similarity_algorithms.h — all-pairs cosine coefficients
    // ------------------------------------------------------------------------

    pub fn cugraph_all_pairs_cosine_similarity_coefficients(
        handle: *const cugraph_resource_handle_t,
        graph: *mut cugraph_graph_t,
        vertices: *const cugraph_type_erased_device_array_view_t,
        use_weight: bool_t,
        topk: usize,
        do_expensive_check: bool_t,
        result: *mut *mut cugraph_similarity_result_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;

    // ------------------------------------------------------------------------
    // tree_algorithms.h — Minimum Spanning Tree
    // ------------------------------------------------------------------------

    pub fn cugraph_minimum_spanning_tree(
        handle: *const cugraph_resource_handle_t,
        graph: *mut cugraph_graph_t,
        do_expensive_check: bool_t,
        result: *mut *mut cugraph_induced_subgraph_result_t,
        error: *mut *mut cugraph_error_t,
    ) -> cugraph_error_code_t;
}
