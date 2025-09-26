use easyjsr::{derefernce_jsarg, JSArg, JSArgResult, JSArgType, Opaque, EJR};

struct EasyJSR {
    ejr: EJR,
    // EasyJSR::new()
    // rt.run(js_content)
}

/// Console.log
fn ___print(args: Vec<JSArg>, opaque: Opaque) -> JSArgResult {
    if args.len() == 0 {
        return None;
    }

    let first = derefernce_jsarg(args[0]);
    if first.type_ == JSArgType::
}

impl EasyJSR {
    pub fn new() -> Self {
        let mut ejr = EJR::new();
        


        Self {
            ejr: ejr
        }
    }
}