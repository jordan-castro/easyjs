use easyjsr::{JSArg, JSArgResult, Opaque};

fn js_encode_text(args: Vec<JSArg>, opaque: Opaque) -> JSArgResult {
    // Only use the first arg
    if (args.len() == 0) {
        return None;
    }

    None
}