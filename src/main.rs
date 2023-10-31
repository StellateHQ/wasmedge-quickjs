#![allow(dead_code, unused_imports, unused_must_use)]

use std::borrow::{Borrow, BorrowMut};
use wasmedge_quickjs::*;

fn args_parse() -> (String, Vec<String>) {
    use argparse::ArgumentParser;
    let mut file_path = String::new();
    let mut res_args: Vec<String> = vec![];
    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut file_path)
            .add_argument("file", argparse::Store, "js file")
            .required();
        ap.refer(&mut res_args)
            .add_argument("arg", argparse::List, "arg");
        ap.parse_args_or_exit();
    }
    (file_path, res_args)
}

fn main() {
    use wasmedge_quickjs as q;

   test_base64();
   test_errors();

    let mut rt = q::Runtime::new();
    rt.run_with_context(|ctx| {
        let (file_path, mut rest_arg) = args_parse();
        let code = std::fs::read_to_string(&file_path);
        match code {
            Ok(code) => {
                rest_arg.insert(0, file_path.clone());
                ctx.put_args(rest_arg);
                ctx.eval_module_str(code, &file_path);
            }
            Err(e) => {
                eprintln!("{}", e.to_string());
            }
        }
        ctx.js_loop().unwrap();
    });
}

fn test_base64() {
    use wasmedge_quickjs as q;
    let mut rt = q::Runtime::new();
    rt.run_with_context(|ctx| {
        let code = String::from("btoa('stellate')");
        let r = ctx.eval_global_str(code, false);
        if let JsValue::String(js_string) = r {
            if js_string.as_str() == "c3RlbGxhdGU=" {
                println!("Matched encode value")
            } else {
                println!("Missmatched encode value")
            }
        } else {
            println!("Missmatched encode value")
        }

        let code = String::from("atob('c3RlbGxhdGU=')");
        let r = ctx.eval_global_str(code, false);
        if let JsValue::String(js_string) = r {
            if js_string.as_str() == "stellate" {
                println!("Matched decode value")
            } else {
                println!("Missmatched decode value")
            }
        } else {
            println!("Missmatched decode value")
        }

        let code = String::from("btoa(undefined)");
        let r = ctx.eval_global_str(code, false);
        if let JsValue::String(js_string) = r {
            if js_string.as_str() == "dW5kZWZpbmVk" {
                println!("Matched encode value")
            } else {
                println!("Missmatched encode value")
            }
        } else {
            println!("Missmatched encode value")
        }

        let code = String::from("atob('dW5kZWZpbmVk')");
        let r = ctx.eval_global_str(code, false);
        if let JsValue::String(js_string) = r {
            if js_string.as_str() == "undefined" {
                println!("Matched decode value")
            } else {
                println!("Missmatched decode value")
            }
        } else {
            println!("Missmatched decode value")
        }
    })
}

fn test_errors() {
    use wasmedge_quickjs as q;

    let mut rt = q::Runtime::new();
    rt.run_with_context(|ctx| {
        let code = String::from("const x = {};x.hello.world;");
        let r = ctx.eval_global_str(code, false);
        match r {
            JsValue::Exception(exception) => {
                println!("Exception value:{:?}", exception.get_message());
            }
            _ => {
                println!("return value:{:?}", r);
            }
        }

        let code = String::from("throw new Error('x.hello');");
        let r = ctx.eval_global_str(code, false);
        match r {
            JsValue::Exception(exception) => {
                println!("Exception value:{:?}", exception.get_message());
            }
            _ => {
                println!("return value:{:?}", r);
            }
        }

        let code = String::from("throw 6;");
        let r = ctx.eval_global_str(code, false);
        match r {
            JsValue::Exception(exception) => {
                println!("Exception value:{:?}", exception.get_message());
            }
            _ => {
                println!("return value:{:?}", r);
            }
        }
    });

    let mut rt = q::Runtime::new();
    rt.run_with_context(|ctx| {
        let code = String::from("(() => {
            return { crazy: 'shit here my friend' };
        })()");
        let r = ctx.eval_global_str(code, false);
        match r {
            JsValue::Exception(exception) => {
                println!("Exception value:{:?}", exception.get_message());
            }
            _ => {
                println!("return value:{:?}", r);
            }
        }
    });
}
