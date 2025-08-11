use encoding_rs::{Encoding, UTF_8, UTF_16LE};
use rquickjs::{
    Class, Context, Ctx, FromJs, Function, IntoJs, JsLifetime, Object, Result, TypedArray,
    class::{JsClass, Readable, Trace, Tracer},
    function::Constructor,
    prelude::This,
};

use crate::add_struct_to_quickjs;

/// A TextEncoder for easyjsr.
///
/// Supports:
/// - utf-8
/// - utf-16
#[derive(Clone, Copy)]
struct TextDecoder {
    pub encoding: &'static Encoding,
}

impl TextDecoder {
    pub fn new(encoding_name: Option<String>) -> Self {
        let encoding = match encoding_name.as_deref() {
            Some("utf-16") => UTF_16LE,
            _ => UTF_8, // default
        };

        TextDecoder { encoding }
    }

    /// Decode a Uint8Array.
    pub fn decode<'js>(&self, input: TypedArray<'js, u8>) -> Result<String> {
        // Decript
        let bytes = input.as_bytes().unwrap();
        let (cow, _, _) = self.encoding.decode(bytes);

        Ok(cow.into_owned())
    }
}

add_struct_to_quickjs!(TextDecoder);


impl<'js> JsClass<'js> for TextDecoder {
    const NAME: &'static str = "TextDecoder";
    const CALLABLE: bool = false;
    type Mutable = Readable;

    fn constructor(ctx: &Ctx<'js>) -> Result<Option<rquickjs::function::Constructor<'js>>> {
        let constr =
            Constructor::new_class::<TextDecoder, _, _>(ctx.clone(), |encoding_name: String| {
                TextDecoder::new(Some(encoding_name))
            })?;

        Ok(Some(constr))
    }
    fn prototype(ctx: &Ctx<'js>) -> Result<Option<rquickjs::Object<'js>>> {
        let proto = Object::new(ctx.clone())?;
        let decode_func = Function::new(ctx.clone(), |this: This<TextDecoder>, input: TypedArray<'js, u8>| {
            this.decode(input)
        })?.with_name("decode");

        proto.set("decode", decode_func)?;
        Ok(Some(proto))
    }
}

/// Add the `TextEncoder` struct to easyjsr context scope.
pub fn add_text_decoder(context: &Context) -> Result<()> {
    context.with(|ctx| -> Result<()> {
        Class::<TextDecoder>::define(&ctx.globals()).unwrap();
        Ok(())
    })
}
