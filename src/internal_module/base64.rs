extern crate base64;
use base64::{Engine as _, engine::general_purpose};

use crate::{JsValue, Context, JsString};
use crate::quickjs_sys::*;

fn atob(ctx: &mut Context, _this_val: JsValue, argv: &[JsValue]) -> JsValue {
    let base64_string = match argv.get(0) {
      Some(JsValue::String(base64_string)) => Some(base64_string),
      // For correctness we would need to convert any number, undefined or null to 
      // a string but not sure if we can coerce here.
      Some(_value) => {
        ctx.throw_type_error("atob needs a string-argument to be passed");
        return JsValue::UnDefined;
      }
      None => {
        ctx.throw_type_error("atob needs an argument to be passed");
        return JsValue::UnDefined;
      }
    };
    if let Some(base64_string) = base64_string {
      let result = general_purpose::STANDARD.decode(base64_string.to_string());
      match result {
        Ok(decoded) => {
          let result = String::from_utf8(decoded);
          match result {
            Ok(final_decoded_string) => {
              let js_string = ctx.new_string(final_decoded_string.as_str());
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
    } else {
        JsValue::UnDefined
    }
}

fn btoa(ctx: &mut Context, _this_val: JsValue, argv: &[JsValue]) -> JsValue {
    let raw_string = argv.get(0);
    if let Some(JsValue::String(raw_string)) = raw_string {
      let encoded_string = general_purpose::STANDARD.encode(raw_string.to_string());
      let js_string = ctx.new_string(encoded_string.as_str());
      JsValue::String(js_string)
    } else {
      ctx.throw_type_error("atob needs a string argument to be passed");
      return JsValue::UnDefined;
    }
}

pub fn init_base64_functions(ctx: &mut Context) {
  let mut global = ctx.get_global();

  global.set("btoa", ctx.wrap_function("btoa", btoa).into());
  global.set("atob", ctx.wrap_function("atob", atob).into());
}