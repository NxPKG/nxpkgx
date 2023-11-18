use std::io::Write;

use anyhow::Result;
use nxpkg_tasks::Vc;
use nxpkg_tasks_env::ProcessEnv;
use nxpkg_tasks_fs::{rope::RopeBuilder, File, FileSystemPath};
use nxpkgpack_core::{
    asset::{Asset, AssetContent},
    ident::AssetIdent,
    source::Source,
};
use nxpkgpack_ecmascript::utils::StringifyJs;

/// The `process.env` asset, responsible for initializing the env (shared by all
/// chunks) during app startup.
#[nxpkg_tasks::value]
pub struct ProcessEnvAsset {
    /// The root path which we can construct our env asset path.
    root: Vc<FileSystemPath>,

    /// A HashMap filled with the env key/values.
    env: Vc<Box<dyn ProcessEnv>>,
}

#[nxpkg_tasks::value_impl]
impl ProcessEnvAsset {
    #[nxpkg_tasks::function]
    pub fn new(root: Vc<FileSystemPath>, env: Vc<Box<dyn ProcessEnv>>) -> Vc<Self> {
        ProcessEnvAsset { root, env }.cell()
    }
}

#[nxpkg_tasks::value_impl]
impl Source for ProcessEnvAsset {
    #[nxpkg_tasks::function]
    fn ident(&self) -> Vc<AssetIdent> {
        AssetIdent::from_path(self.root.join(".env.js".to_string()))
    }
}

#[nxpkg_tasks::value_impl]
impl Asset for ProcessEnvAsset {
    #[nxpkg_tasks::function]
    async fn content(&self) -> Result<Vc<AssetContent>> {
        let env = self.env.read_all().await?;

        // TODO: In SSR, we use the native process.env, which can only contain string
        // values. We need to inject literal values (to emulate webpack's
        // DefinePlugin), so create a new regular object out of the old env.
        let mut code = RopeBuilder::default();
        code += "const env = process.env = {...process.env};\n\n";

        for (name, val) in &*env {
            // It's assumed the env has passed through an EmbeddableProcessEnv, so the value
            // is ready to be directly embedded. Values _after_ an embeddable
            // env can be used to inject live code into the output.
            // TODO this is not completely correct as env vars need to ignore casing
            // So `process.env.path === process.env.PATH === process.env.PaTh`
            writeln!(code, "env[{}] = {};", StringifyJs(name), val)?;
        }

        Ok(AssetContent::file(File::from(code.build()).into()))
    }
}
