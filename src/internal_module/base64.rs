use crate::quickjs_sys::*;
use crate::{Context, JsString, JsValue};
use base64::{engine::general_purpose, Engine as _};

fn coerce_to_string(ctx: &mut Context, js_value: &JsValue) -> Option<JsString> {
    match js_value {
        JsValue::String(value) => Some(value.to_owned()),
        JsValue::Int(number) => Some(ctx.new_string(number.to_string().as_str())),
        JsValue::Bool(bool) => Some(ctx.new_string(bool.to_string().as_str())),
        JsValue::Float(float) => Some(ctx.new_string(float.to_string().as_str())),
        JsValue::UnDefined => Some(ctx.new_string("undefined")),
        JsValue::Null => Some(ctx.new_string("null")),
        _ => None,
    }
}

fn atob(ctx: &mut Context, _this_val: JsValue, argv: &[JsValue]) -> JsValue {
    let base64_string = match argv.get(0) {
        Some(JsValue::String(base64_string)) => base64_string.as_str(),
        _ => {
            ctx.throw_type_error("atob needs an argument to be passed");
            return JsValue::UnDefined;
        }
    };

    let result = general_purpose::STANDARD
        .decode(base64_string)
        .or_else(|_| base64::engine::general_purpose::STANDARD_NO_PAD.decode(base64_string));
    match result {
        Ok(decoded) => {
            let result = String::from_utf8(decoded);
            match result {
                Ok(final_decoded_string) => {
                    let js_string = ctx.new_string(&final_decoded_string);
                    JsValue::String(js_string)
                }
                Err(_e) => {
                    ctx.throw_type_error("Could not decode to UTF-8 String");
                    JsValue::UnDefined
                }
            }
        }

        Err(_e) => {
            ctx.throw_type_error("Could not decode to UTF-8 String");
            JsValue::UnDefined
        }
    }
}

fn btoa(ctx: &mut Context, _this_val: JsValue, argv: &[JsValue]) -> JsValue {
    let js_string = match argv.get(0) {
        Some(JsValue::String(base64_string)) => base64_string.to_owned(),

        Some(value) => match coerce_to_string(ctx, value) {
            Some(result) => result,
            None => {
                ctx.throw_type_error("btoa needs a string-argument to be passed");
                return JsValue::UnDefined;
            }
        },

        None => {
            ctx.throw_type_error("btoa needs an argument to be passed");
            return JsValue::UnDefined;
        }
    };

    let encoded_string = general_purpose::STANDARD.encode(js_string.as_str());
    JsValue::String(ctx.new_string(&encoded_string))
}

pub fn init_base64_functions(ctx: &mut Context) {
    let mut global = ctx.get_global();

    global.set("btoa", ctx.wrap_function("btoa", btoa).into());
    global.set("atob", ctx.wrap_function("atob", atob).into());
}
