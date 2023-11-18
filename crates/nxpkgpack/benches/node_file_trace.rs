use std::{collections::HashMap, fs, path::PathBuf};

use criterion::{Bencher, BenchmarkId, Criterion};
use regex::Regex;
use nxpkg_tasks::{NxpkgTasks, Value, Vc};
use nxpkg_tasks_fs::{DiskFileSystem, FileSystem, NullFileSystem};
use nxpkg_tasks_memory::MemoryBackend;
use nxpkgpack::{
    emit_with_completion, module_options::ModuleOptionsContext, rebase::RebasedAsset, register,
    resolve_options_context::ResolveOptionsContext, ModuleAssetContext,
};
use nxpkgpack_core::{
    compile_time_info::CompileTimeInfo,
    context::AssetContext,
    environment::{Environment, ExecutionEnvironment, NodeJsEnvironment},
    file_source::FileSource,
    reference_type::ReferenceType,
};

// TODO this should move to the `node-file-trace` crate
pub fn benchmark(c: &mut Criterion) {
    register();

    let bench_filter = Regex::new(r"(empty|simple|dynamic-in-package|react|whatwg-url|axios|azure-cosmos|cowsay|env-var|fast-glob)\.js$").unwrap();

    let tests_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests");
    let tests_dir = tests_root.join("node-file-trace/integration");

    let mut group = c.benchmark_group("node-file-trace");
    group.sample_size(10);

    let results = fs::read_dir(tests_dir).unwrap();
    for result in results {
        let entry = result.unwrap();
        if entry.file_type().unwrap().is_file() {
            let name = entry.file_name().into_string().unwrap();
            if !bench_filter.is_match(&name) {
                continue;
            }

            let input = format!("node-file-trace/integration/{name}");
            let tests_root = tests_root.to_string_lossy().to_string();

            let bench_input = BenchInput { tests_root, input };

            group.bench_with_input(
                BenchmarkId::new("emit", &bench_input.input),
                &bench_input,
                bench_emit,
            );
        }
    }

    group.finish();
}

struct BenchInput {
    tests_root: String,
    input: String,
}

fn bench_emit(b: &mut Bencher, bench_input: &BenchInput) {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    b.to_async(rt).iter(move || {
        let tt = NxpkgTasks::new(MemoryBackend::default());
        let tests_root = bench_input.tests_root.clone();
        let input = bench_input.input.clone();
        async move {
            let task = tt.spawn_once_task(async move {
                let input_fs = DiskFileSystem::new("tests".to_string(), tests_root.clone());
                let input = input_fs.root().join(input.clone());

                let input_dir = input.parent().parent();
                let output_fs: Vc<NullFileSystem> = NullFileSystem.into();
                let output_dir = output_fs.root();

                let source = FileSource::new(input);
                let compile_time_info = CompileTimeInfo::builder(Environment::new(Value::new(
                    ExecutionEnvironment::NodeJsLambda(NodeJsEnvironment::default().into()),
                )))
                .cell();
                let module_asset_context = ModuleAssetContext::new(
                    Vc::cell(HashMap::new()),
                    compile_time_info,
                    ModuleOptionsContext {
                        enable_types: true,
                        ..Default::default()
                    }
                    .cell(),
                    ResolveOptionsContext {
                        emulate_environment: Some(compile_time_info.environment().resolve().await?),
                        ..Default::default()
                    }
                    .cell(),
                    Vc::cell("node_file_trace".to_string()),
                );
                let module = module_asset_context
                    .process(Vc::upcast(source), Value::new(ReferenceType::Undefined));
                let rebased = RebasedAsset::new(Vc::upcast(module), input_dir, output_dir);

                emit_with_completion(Vc::upcast(rebased), output_dir).await?;

                Ok::<Vc<()>, _>(Default::default())
            });
            tt.wait_task_completion(task, true).await.unwrap();
        }
    })
}
