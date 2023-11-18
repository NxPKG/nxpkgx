#![feature(future_join)]
#![feature(min_specialization)]

use std::path::Path;

use anyhow::{Context, Result};
use clap::Parser;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Registry};
use nxpkgpack_cli::{arguments::Arguments, register};
use nxpkgpack_cli_utils::{
    exit::ExitGuard,
    raw_trace::RawTraceLayer,
    trace_writer::TraceWriter,
    tracing_presets::{
        TRACING_OVERVIEW_TARGETS, TRACING_NXPKGPACK_TARGETS, TRACING_NXPKG_TASKS_TARGETS,
    },
};

#[global_allocator]
static ALLOC: nxpkg_tasks_malloc::NxpkgMalloc = nxpkg_tasks_malloc::NxpkgMalloc;

fn main() {
    use nxpkg_tasks_malloc::NxpkgMalloc;

    let args = Arguments::parse();

    let trace = std::env::var("NXPKGPACK_TRACING").ok();

    let _guard = if let Some(mut trace) = trace {
        // Trace presets
        match trace.as_str() {
            "overview" => {
                trace = TRACING_OVERVIEW_TARGETS.join(",");
            }
            "nxpkgpack" => {
                trace = TRACING_NXPKGPACK_TARGETS.join(",");
            }
            "nxpkg-tasks" => {
                trace = TRACING_NXPKG_TASKS_TARGETS.join(",");
            }
            _ => {}
        }

        let subscriber = Registry::default();

        let subscriber = subscriber.with(EnvFilter::builder().parse(trace).unwrap());

        let internal_dir = args
            .dir()
            .unwrap_or_else(|| Path::new("."))
            .join(".nxpkgpack");
        std::fs::create_dir_all(&internal_dir)
            .context("Unable to create .nxpkgpack directory")
            .unwrap();
        let trace_file = internal_dir.join("trace.log");
        let trace_writer = std::fs::File::create(trace_file).unwrap();
        let (trace_writer, guard) = TraceWriter::new(trace_writer);
        let subscriber = subscriber.with(RawTraceLayer::new(trace_writer));

        let guard = ExitGuard::new(guard).unwrap();

        subscriber.init();

        Some(guard)
    } else {
        None
    };

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .on_thread_stop(|| {
            NxpkgMalloc::thread_stop();
        })
        .build()
        .unwrap()
        .block_on(main_inner(args))
        .unwrap();
}

async fn main_inner(args: Arguments) -> Result<()> {
    register();

    match args {
        Arguments::Build(args) => nxpkgpack_cli::build::build(&args).await,
        Arguments::Dev(args) => nxpkgpack_cli::dev::start_server(&args).await,
    }
}
