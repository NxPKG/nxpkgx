use once_cell::sync::Lazy;

pub static TRACING_OVERVIEW_TARGETS: Lazy<Vec<&str>> = Lazy::new(|| {
    vec![
        "nxpkg_tasks_fs=info",
        "nxpkgpack_dev_server=info",
        "nxpkgpack_node=info",
    ]
});
pub static TRACING_NXPKGPACK_TARGETS: Lazy<Vec<&str>> = Lazy::new(|| {
    [
        &TRACING_OVERVIEW_TARGETS[..],
        &[
            "nxpkg_tasks=info",
            "nxpkgpack=trace",
            "nxpkgpack_core=trace",
            "nxpkgpack_ecmascript=trace",
            "nxpkgpack_css=trace",
            "nxpkgpack_dev=trace",
            "nxpkgpack_image=trace",
            "nxpkgpack_dev_server=trace",
            "nxpkgpack_json=trace",
            "nxpkgpack_mdx=trace",
            "nxpkgpack_node=trace",
            "nxpkgpack_static=trace",
            "nxpkgpack_cli_utils=trace",
            "nxpkgpack_cli=trace",
            "nxpkgpack_ecmascript=trace",
        ],
    ]
    .concat()
});
pub static TRACING_NXPKG_TASKS_TARGETS: Lazy<Vec<&str>> = Lazy::new(|| {
    [
        &TRACING_NXPKGPACK_TARGETS[..],
        &[
            "nxpkg_tasks=trace",
            "nxpkg_tasks_viz=trace",
            "nxpkg_tasks_memory=trace",
            "nxpkg_tasks_fs=trace",
        ],
    ]
    .concat()
});
