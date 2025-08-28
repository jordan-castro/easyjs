use rquickjs::{loader::{
    FileResolver, ModuleLoader, NativeLoader, ScriptLoader,
}, Runtime};


/// Set the runtimes loaders
pub fn set_easyjsr_module_loader(rt: &Runtime) {
    let resolver = (
        FileResolver::default().with_path("./").with_native(),
    );
    let loader = (
        ScriptLoader::default(),
        NativeLoader::default(),
        ModuleLoader::default(),
    );

    rt.set_loader(resolver, loader);
}