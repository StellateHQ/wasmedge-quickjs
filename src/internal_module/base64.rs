extern crate base64;
use base64::{Engine as _, engine::general_purpose};

use crate::{JsValue, Context, JsString};
use crate::quickjs_sys::*;

fn atob(ctx: &mut Context, _this_val: JsValue, argv: &[JsValue]) -> JsValue {
    let base64_string = argv.get(0);
    if let Some(JsValue::String(base64_string)) = base64_string {
      let result = general_purpose::STANDARD_NO_PAD.decode(base64_string.into());
      if let Ok(decoded) = result {
        let result = String::from_utf8(decoded);
        if let Ok(final_decoded_string) = result {
          // TODO: needs to become JsString
          JsValue::String(final_decoded_string.into())
        } else {
          JsValue::UnDefined
        }
      } else {
        JsValue::UnDefined
      }
    } else {
        JsValue::UnDefined
    }
}

fn btoa(ctx: &mut Context, _this_val: JsValue, argv: &[JsValue]) -> JsValue {
    let raw_string = argv.get(0);
    if let Some(JsValue::String(raw_string)) = raw_string {
      let result = general_purpose::STANDARD_NO_PAD.encode(raw_string.into());
      if let Ok(encoded) = result {
        JsValue::String(encoded)
      } else {
        JsValue::UnDefined
      }
    } else {
        JsValue::UnDefined
    }
}

pub fn init_base64_functions(ctx: &mut Context) {
  let mut global = ctx.get_global();

  global.set("btoa", ctx.wrap_function("btoa", btoa).into());
  global.set("atob", ctx.wrap_function("atob", atob).into());
}