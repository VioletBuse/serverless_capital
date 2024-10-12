use std::{borrow::Cow, cell::RefCell, collections::HashMap, ffi::OsString, rc::Rc, str::FromStr};

use anyhow::{anyhow, bail};
use deno_ast::MediaType;
use deno_core::{
    error::AnyError, resolve_import, ModuleCodeBytes, ModuleLoadResponse, ModuleLoader,
    ModuleSource, ModuleSourceCode, ModuleSpecifier, ModuleType,
};
use std::fs;

use crate::tenant::Tenant;

type SourceMapStore = Rc<RefCell<HashMap<String, Vec<u8>>>>;

pub struct CustomModuleLoader {
    tenant: Tenant,
    source_maps: SourceMapStore,
}

impl CustomModuleLoader {
    pub fn new(tenant: Tenant) -> Self {
        Self {
            tenant,
            source_maps: Rc::new(RefCell::new(HashMap::new())),
        }
    }
}

impl ModuleLoader for CustomModuleLoader {
    fn resolve(
        &self,
        specifier: &str,
        referrer: &str,
        kind: deno_core::ResolutionKind,
    ) -> Result<deno_core::ModuleSpecifier, anyhow::Error> {
        Ok(resolve_import(specifier, referrer)?)
    }

    fn load(
        &self,
        module_specifier: &deno_core::ModuleSpecifier,
        maybe_referrer: Option<&deno_core::ModuleSpecifier>,
        is_dyn_import: bool,
        requested_module_type: deno_core::RequestedModuleType,
    ) -> deno_core::ModuleLoadResponse {
        let source_maps = self.source_maps.clone();
        fn load(
            source_maps: SourceMapStore,
            module_specifier: &ModuleSpecifier,
        ) -> Result<ModuleSource, AnyError> {
            let path = module_specifier
                .to_file_path()
                .map_err(|_| anyhow!("Only file:// URLs are supported"))?;

            let media_type = MediaType::from_path(&path);
            let (module_type, should_transpile) = match MediaType::from_path(&path) {
                MediaType::JavaScript | MediaType::Mjs | MediaType::Cjs => {
                    (ModuleType::JavaScript, false)
                }
                MediaType::Jsx => (ModuleType::JavaScript, true),
                MediaType::TypeScript
                | MediaType::Mts
                | MediaType::Cts
                | MediaType::Dts
                | MediaType::Dmts
                | MediaType::Dcts
                | MediaType::Tsx => (ModuleType::JavaScript, true),
                MediaType::Json => (ModuleType::Json, false),
                MediaType::Wasm => (ModuleType::Wasm, false),
                MediaType::TsBuildInfo | MediaType::SourceMap => (
                    ModuleType::Other(Cow::from("Build data: ts_build_info or source_map")),
                    false,
                ),
                MediaType::Unknown => bail!(
                    "Unknown module extension '{:?}'",
                    path.extension()
                        .unwrap_or(OsString::from_str("NO_EXT")?.as_os_str())
                ),
            };

            let source: ModuleSourceCode = match module_type {
                ModuleType::Wasm => {
                    let module_source = fs::read(&path)?;
                    ModuleSourceCode::Bytes(ModuleCodeBytes::Boxed(module_source.into()))
                }
                _ => {
                    let source = fs::read_to_string(&path)?;
                    if should_transpile {
                        let parsed = deno_ast::parse_module(deno_ast::ParseParams {
                            specifier: module_specifier.clone(),
                            text: source.into(),
                            media_type,
                            capture_tokens: false,
                            scope_analysis: false,
                            maybe_syntax: None,
                        })?;
                        let res = parsed.transpile(
                            &deno_ast::TranspileOptions {
                                imports_not_used_as_values:
                                    deno_ast::ImportsNotUsedAsValues::Remove,
                                use_decorators_proposal: true,
                                ..Default::default()
                            },
                            &deno_ast::EmitOptions {
                                source_map: deno_ast::SourceMapOption::Separate,
                                inline_sources: true,
                                ..Default::default()
                            },
                        )?;

                        let res = res.into_source();
                        let source_map = res.source_map;

                        if let Some(sm) = source_map {
                            source_maps
                                .borrow_mut()
                                .insert(module_specifier.to_string(), sm);
                        }

                        ModuleSourceCode::String(String::from_utf8(res.source)?.into())
                    } else {
                        ModuleSourceCode::String(source.into())
                    }
                }
            };

            Ok(ModuleSource::new(
                module_type,
                source,
                module_specifier,
                None,
            ))
        }

        ModuleLoadResponse::Sync(load(source_maps, module_specifier))
    }

    fn get_source_map(&self, file_name: &str) -> Option<Vec<u8>> {
        self.source_maps.borrow().get(file_name).cloned()
    }
}
