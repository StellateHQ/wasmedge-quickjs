extern crate base64;
use base64::{Engine as _, engine::general_purpose};

use crate::{JsValue, Context, JsString};
use crate::quickjs_sys::*;

fn coerce_to_string(ctx: &mut Context, js_value: &JsValue) -> Option<JsString> {
  match js_value {
    JsValue::String(value) => Some(value.to_owned()),
    JsValue::Int(number) => Some(ctx.new_string(number.to_string().as_str())),
    JsValue::Bool(bool) => Some(ctx.new_string(bool.to_string().as_str())),
    JsValue::Float(float) => Some(ctx.new_string(float.to_string().as_str())),
    JsValue::UnDefined => Some(ctx.new_string("undefined")),
    JsValue::Null => Some(ctx.new_string("null")),
    _ => None
  }
}

fn atob(ctx: &mut Context, _this_val: JsValue, argv: &[JsValue]) -> JsValue {
    let base64_string = match argv.get(0) {
      Some(JsValue::String(base64_string)) => base64_string.to_owned(),
      _ => {
        ctx.throw_type_error("atob needs an argument to be passed");
        return JsValue::UnDefined;
      }
    };

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
}

fn btoa(ctx: &mut Context, _this_val: JsValue, argv: &[JsValue]) -> JsValue {
  let raw_string = match argv.get(0) {
    Some(JsValue::String(base64_string)) => base64_string.to_owned(),
    // For correctness we would need to convert any number, undefined or null to 
    // a string but not sure if we can coerce here.
    Some(value) => {
      let result = coerce_to_string(ctx, value);
      if let Some(result) = result {
        result
      } else {
        ctx.throw_type_error("btoa needs a string-argument to be passed");
        return JsValue::UnDefined;
      }
    }
    None => {
      ctx.throw_type_error("btoa needs an argument to be passed");
      return JsValue::UnDefined;
    }
  };

    let encoded_string = general_purpose::STANDARD.encode(raw_string.to_string());
    let js_string = ctx.new_string(encoded_string.as_str());
    JsValue::String(js_string)
}

pub fn init_base64_functions(ctx: &mut Context) {
  let mut global = ctx.get_global();

  global.set("btoa", ctx.wrap_function("btoa", btoa).into());
  global.set("atob", ctx.wrap_function("atob", atob).into());
}