use anyhow::Result;
use nxpkg_tasks::Vc;

use crate::{case_insensitive_read, EnvMap, ProcessEnv};

/// Allows providing any custom env values that you'd like, deferring the prior
/// envs if a key is not overridden.
#[nxpkg_tasks::value]
pub struct CustomProcessEnv {
    prior: Vc<Box<dyn ProcessEnv>>,
    custom: Vc<EnvMap>,
}

#[nxpkg_tasks::value_impl]
impl CustomProcessEnv {
    #[nxpkg_tasks::function]
    pub fn new(prior: Vc<Box<dyn ProcessEnv>>, custom: Vc<EnvMap>) -> Vc<Self> {
        CustomProcessEnv { prior, custom }.cell()
    }
}

#[nxpkg_tasks::value_impl]
impl ProcessEnv for CustomProcessEnv {
    #[nxpkg_tasks::function]
    async fn read_all(&self) -> Result<Vc<EnvMap>> {
        let prior = self.prior.read_all().await?;
        let custom = self.custom.await?;

        let mut extended = prior.clone_value();
        extended.extend(custom.clone_value());
        Ok(Vc::cell(extended))
    }

    #[nxpkg_tasks::function]
    async fn read(&self, name: String) -> Result<Vc<Option<String>>> {
        let custom = case_insensitive_read(self.custom, name.clone());
        match &*custom.await? {
            Some(_) => Ok(custom),
            None => Ok(self.prior.read(name)),
        }
    }
}
