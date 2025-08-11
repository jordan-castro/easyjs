/// We constantly need to get the globalThis object. So use this macro, just pass in the context.
#[macro_export]
macro_rules! get_global {
    ($ctx:expr) => {
        $ctx.globals()
    };
}

/// Add a new function to a `Object`
#[macro_export]
macro_rules! add_function_to_object {
    ($context:expr, $global:expr, $func:expr, $name:expr) => {
        $global.set(
            $name,
            Function::new($context.clone(), $func)?.with_name($name)?,
        )?;
    };
}

/// Add a new struct to quickjs.
#[macro_export]
macro_rules! add_struct_to_quickjs {
    ($struct_type:ty) => {
        impl<'js> Trace<'js> for $struct_type {
            fn trace<'a>(&self, _tracer: Tracer<'a, 'js>) {}
        }

        impl <'js> IntoJs<'js> for $struct_type {
            fn into_js(self, ctx: &Ctx<'js>) -> Result<rquickjs::Value<'js>> {
                Class::instance(ctx.clone(), self).into_js(ctx)
            }
        }

        impl <'js> FromJs<'js> for $struct_type {
            fn from_js(ctx: &Ctx<'js>, value: rquickjs::Value<'js>) -> Result<Self> {
                Ok(*Class::<$struct_type>::from_js(ctx, value)?.try_borrow()?)
            }
        }

        unsafe impl<'js> JsLifetime<'js> for $struct_type {
            type Changed<'to> = $struct_type;
        }
    };
}