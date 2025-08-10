use encoding_rs::{Encoding, UTF_8, UTF_16LE};
use rquickjs::{
    Class, Context, Ctx, FromJs, Function, IntoJs, JsLifetime, Object, Result, TypedArray,
    class::{JsClass, Readable, Trace, Tracer},
    function::Constructor,
    prelude::This,
};

/// A TextEncoder for easyjsr.
///
/// Supports:
/// - utf-8
/// - utf-16
#[derive(Clone, Copy)]
struct TextEncoder {
    pub encoding: &'static Encoding,
}

impl TextEncoder {
    pub fn new(encoding_name: Option<String>) -> Self {
        let encoding = match encoding_name.as_deref() {
            Some("utf-16") => UTF_16LE,
            _ => UTF_8, // default
        };

        TextEncoder { encoding }
    }

    /// Encode a string.
    pub fn encode<'js>(&self, ctx: Ctx<'js>, input: String) -> Result<TypedArray<'js, u8>> {
        // encoding_rs returns (Cow<[u8]>, ..)
        let (bytes_cow, _, _) = self.encoding.encode(&input);
        // ensure we own bytes as Vec<u8>
        let bytes: Vec<u8> = bytes_cow.into_owned();

        // Create a Uint8Array directly from the Vec<u8>
        // TypedArray::new(ctx, vec) constructs a TypedArray backed by those bytes.
        TypedArray::<u8>::new(ctx, bytes)
    }

    /// Decode a string.
    pub fn decode<'js>(&self, ctx: Ctx<'js>, input: TypedArray<u8>) -> Result<String> {
        // Decript
        let result = self.encoding.decode(input)

        Ok("".to_string())
    }
}

impl<'js> Trace<'js> for TextEncoder {
    fn trace<'a>(&self, _tracer: Tracer<'a, 'js>) {}
}

impl<'js> IntoJs<'js> for TextEncoder {
    fn into_js(self, ctx: &Ctx<'js>) -> Result<rquickjs::Value<'js>> {
        Class::instance(ctx.clone(), self).into_js(ctx)
    }
}

impl<'js> FromJs<'js> for TextEncoder {
    fn from_js(ctx: &Ctx<'js>, value: rquickjs::Value<'js>) -> Result<Self> {
        Ok(*Class::<TextEncoder>::from_js(ctx, value)?.try_borrow()?)
    }
}
unsafe impl<'js> JsLifetime<'js> for TextEncoder {
    type Changed<'to> = TextEncoder;
}

impl<'js> JsClass<'js> for TextEncoder {
    const NAME: &'static str = "TextEncoder";
    const CALLABLE: bool = false;
    type Mutable = Readable;

    fn constructor(ctx: &Ctx<'js>) -> Result<Option<rquickjs::function::Constructor<'js>>> {
        let constr =
            Constructor::new_class::<TextEncoder, _, _>(ctx.clone(), |encoding_name: String| {
                TextEncoder::new(Some(encoding_name))
            })?;

        Ok(Some(constr))
    }
    fn prototype(ctx: &Ctx<'js>) -> Result<Option<rquickjs::Object<'js>>> {
        let proto = Object::new(ctx.clone())?;
        let func = Function::new(ctx.clone(), |ctx: Ctx<'js>, this: This<TextEncoder>, input: String| {
            this.encode(ctx, input)
        })?
        .with_name("encode")?;

        proto.set("encode", func)?;
        Ok(Some(proto))
    }
}

/// Add the `TextEncoder` struct to easyjsr context scope.
pub fn add_text_encoder(context: &Context) -> Result<()> {
    context.with(|ctx| -> Result<()> {
        Class::<TextEncoder>::define(&ctx.globals()).unwrap();
        Ok(())
    })
}
