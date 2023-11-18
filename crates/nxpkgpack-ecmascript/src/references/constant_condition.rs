use anyhow::Result;
use swc_core::quote;
use nxpkg_tasks::{Value, Vc};

use super::AstPath;
use crate::{
    chunk::EcmascriptChunkingContext,
    code_gen::{CodeGenerateable, CodeGeneration},
    create_visitor,
};

#[nxpkg_tasks::value(serialization = "auto_for_input")]
#[derive(Debug, Clone, Copy, Hash, PartialOrd, Ord)]
pub enum ConstantConditionValue {
    Truthy,
    Falsy,
    Nullish,
}

#[nxpkg_tasks::value]
pub struct ConstantCondition {
    value: ConstantConditionValue,
    path: Vc<AstPath>,
}

#[nxpkg_tasks::value_impl]
impl ConstantCondition {
    #[nxpkg_tasks::function]
    pub fn new(value: Value<ConstantConditionValue>, path: Vc<AstPath>) -> Vc<Self> {
        Self::cell(ConstantCondition {
            value: value.into_value(),
            path,
        })
    }
}

#[nxpkg_tasks::value_impl]
impl CodeGenerateable for ConstantCondition {
    #[nxpkg_tasks::function]
    async fn code_generation(
        &self,
        _context: Vc<Box<dyn EcmascriptChunkingContext>>,
    ) -> Result<Vc<CodeGeneration>> {
        let value = self.value;
        let visitors = [
            create_visitor!(exact &self.path.await?, visit_mut_expr(expr: &mut Expr) {
                *expr = match value {
                    ConstantConditionValue::Truthy => quote!("(\"NXPKGPACK compile-time truthy\", 1)" as Expr),
                    ConstantConditionValue::Falsy => quote!("(\"NXPKGPACK compile-time falsy\", 0)" as Expr),
                    ConstantConditionValue::Nullish => quote!("(\"NXPKGPACK compile-time nullish\", null)" as Expr),
                };
            }),
        ]
        .into();

        Ok(CodeGeneration { visitors }.cell())
    }
}
