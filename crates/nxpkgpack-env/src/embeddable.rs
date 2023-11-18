use anyhow::Result;
use nxpkg_tasks::Vc;
use nxpkg_tasks_env::{EnvMap, ProcessEnv};
use nxpkgpack_ecmascript::utils::StringifyJs;

/// Encodes values as JS strings so that they can be safely injected into a JS
/// output.
#[nxpkg_tasks::value]
pub struct EmbeddableProcessEnv {
    prior: Vc<Box<dyn ProcessEnv>>,
}

#[nxpkg_tasks::value_impl]
impl EmbeddableProcessEnv {
    #[nxpkg_tasks::function]
    pub fn new(prior: Vc<Box<dyn ProcessEnv>>) -> Vc<Self> {
        EmbeddableProcessEnv { prior }.cell()
    }
}

#[nxpkg_tasks::value_impl]
impl ProcessEnv for EmbeddableProcessEnv {
    #[nxpkg_tasks::function]
    async fn read_all(&self) -> Result<Vc<EnvMap>> {
        let prior = self.prior.read_all().await?;

        let encoded = prior
            .iter()
            .map(|(k, v)| (k.clone(), StringifyJs(v).to_string()))
            .collect();

        Ok(Vc::cell(encoded))
    }

    #[nxpkg_tasks::function]
    async fn read(&self, name: String) -> Result<Vc<Option<String>>> {
        let prior = self.prior.read(name).await?;
        let encoded = prior.as_deref().map(|s| StringifyJs(s).to_string());
        Ok(Vc::cell(encoded))
    }
}
