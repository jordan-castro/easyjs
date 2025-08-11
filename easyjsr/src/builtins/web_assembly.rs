use rquickjs::{
    Class, Context, Ctx, FromJs, Function, IntoJs, JsLifetime, Object, Result, TypedArray,
    class::{JsClass, Readable, Trace, Tracer},
    function::Constructor,
    prelude::This,
};

/// A WebAssembly.Module rust implementation for easyjsr.
/// Works with rquickjs.
#[derive(Clone)]
struct WebAssemblyModule<'js> {
    buffer_source: TypedArray<'js, u8>
}
