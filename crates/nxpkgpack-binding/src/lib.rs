#[cfg(feature = "__swc")]
pub mod swc {
    #[cfg(feature = "__swc_core")]
    pub use swc_core as core;

    #[cfg(feature = "__swc_custom_transform")]
    pub mod custom_transform {
        #[cfg(feature = "__swc_transform_modularize_imports")]
        pub use modularize_imports;
        #[cfg(feature = "__swc_transform_styled_components")]
        pub use styled_components;
        #[cfg(feature = "__swc_transform_styled_jsx")]
        pub use styled_jsx;
        #[cfg(feature = "__swc_transform_emotion")]
        pub use swc_emotion as emotion;
        #[cfg(feature = "__swc_transform_relay")]
        pub use swc_relay as relay;
    }

    #[cfg(feature = "testing")]
    pub use testing;
}

#[cfg(feature = "__nxpkg")]
pub mod nxpkg {
    #[cfg(feature = "__nxpkg_tasks")]
    pub use nxpkg_tasks as tasks;
    #[cfg(feature = "__nxpkg_tasks_build")]
    pub use nxpkg_tasks_build as tasks_build;
    #[cfg(feature = "__nxpkg_tasks_bytes")]
    pub use nxpkg_tasks_bytes as tasks_bytes;
    #[cfg(feature = "__nxpkg_tasks_env")]
    pub use nxpkg_tasks_env as tasks_env;
    #[cfg(feature = "__nxpkg_tasks_fetch")]
    pub use nxpkg_tasks_fetch as tasks_fetch;
    #[cfg(feature = "__nxpkg_tasks_fs")]
    pub use nxpkg_tasks_fs as tasks_fs;
    #[cfg(feature = "__nxpkg_tasks_hash")]
    pub use nxpkg_tasks_hash as tasks_hash;
    #[cfg(feature = "__nxpkg_tasks_macros")]
    pub use nxpkg_tasks_macros as tasks_macros;
    #[cfg(feature = "__nxpkg_tasks_macros_shared")]
    pub use nxpkg_tasks_macros_shared as tasks_macros_shared;
    #[cfg(feature = "__nxpkg_tasks_malloc")]
    pub use nxpkg_tasks_malloc as malloc;
    #[cfg(feature = "__nxpkg_tasks_memory")]
    pub use nxpkg_tasks_memory as tasks_memory;
    #[cfg(feature = "__nxpkg_tasks_testing")]
    pub use nxpkg_tasks_testing as tasks_testing;
    #[cfg(feature = "__nxpkg_updater")]
    pub use nxpkg_updater as updater;
}

#[cfg(feature = "__nxpkgpack")]
pub mod nxpkgpack {
    pub use nxpkgpack;
    #[cfg(feature = "__nxpkgpack_bench")]
    pub use nxpkgpack_bench as bench;
    #[cfg(feature = "__nxpkgpack_build")]
    pub use nxpkgpack_build as build;
    #[cfg(feature = "__nxpkgpack_cli_utils")]
    pub use nxpkgpack_cli_utils as cli_utils;
    #[cfg(feature = "__nxpkgpack_core")]
    pub use nxpkgpack_core as core;
    #[cfg(feature = "__nxpkgpack_create_test_app")]
    pub use nxpkgpack_create_test_app as create_test_app;
    #[cfg(feature = "__nxpkgpack_css")]
    pub use nxpkgpack_css as css;
    #[cfg(feature = "__nxpkgpack_dev")]
    pub use nxpkgpack_dev as dev;
    #[cfg(feature = "__nxpkgpack_dev_server")]
    pub use nxpkgpack_dev_server as dev_server;
    #[cfg(feature = "__nxpkgpack_ecmascript")]
    pub use nxpkgpack_ecmascript as ecmascript;
    #[cfg(feature = "__nxpkgpack_ecmascript_hmr_protocol")]
    pub use nxpkgpack_ecmascript_hmr_protocol as ecmascript_hmr_protocol;
    #[cfg(feature = "__nxpkgpack_ecmascript_plugin")]
    pub use nxpkgpack_ecmascript_plugins as ecmascript_plugin;
    #[cfg(feature = "__nxpkgpack_ecmascript_runtime")]
    pub use nxpkgpack_ecmascript_runtime as ecmascript_runtime;
    #[cfg(feature = "__nxpkgpack_env")]
    pub use nxpkgpack_env as env;
    #[cfg(feature = "__nxpkgpack_image")]
    pub use nxpkgpack_image as image;
    #[cfg(feature = "__nxpkgpack_json")]
    pub use nxpkgpack_json as json;
    #[cfg(feature = "__nxpkgpack_mdx")]
    pub use nxpkgpack_mdx as mdx;
    #[cfg(feature = "__nxpkgpack_node")]
    pub use nxpkgpack_node as node;
    #[cfg(feature = "__nxpkgpack_static")]
    pub use nxpkgpack_static as r#static;
    #[cfg(feature = "__nxpkgpack_swc_utils")]
    pub use nxpkgpack_swc_utils as swc_utils;
    #[cfg(feature = "__nxpkgpack_test_utils")]
    pub use nxpkgpack_test_utils as test_utils;
    #[cfg(feature = "__nxpkgpack_tests")]
    pub use nxpkgpack_tests as tests;
}

#[cfg(feature = "__features")]
pub mod features {
    #[cfg(feature = "__feature_auto_hash_map")]
    pub use auto_hash_map;
    #[cfg(feature = "__feature_mdx_rs")]
    pub use mdxjs;
    #[cfg(feature = "__feature_node_file_trace")]
    pub use node_file_trace;
    #[cfg(feature = "__feature_swc_ast_explorer")]
    pub use swc_ast_explorer;
    #[cfg(feature = "__feature_tracing_signpost")]
    pub use tracing_signpost;
}
