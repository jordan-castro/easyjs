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