use log::{debug, error, info};
pub struct Console;

impl Console {
    pub fn info(
        scope: &mut v8::HandleScope,
        args: v8::FunctionCallbackArguments,
        mut rv: v8::ReturnValue,
    ) {
        let mut all_args: Vec<String> = vec![];
        for i in 0..args.length() {
            let arg = args.get(i);
            let arg_string = arg.to_string(scope).unwrap().to_rust_string_lossy(scope);
            all_args.push(arg_string);
        }
        info!("{}", all_args.join(" "));

        let returned_value_string = v8::Number::new(scope, 0f64).into();
        rv.set(returned_value_string);
    }
    pub fn error(
        scope: &mut v8::HandleScope,
        args: v8::FunctionCallbackArguments,
        mut rv: v8::ReturnValue,
    ) {
        let mut all_args: Vec<String> = vec![];
        for i in 0..args.length() {
            let arg = args.get(i);
            let arg_string = arg.to_string(scope).unwrap().to_rust_string_lossy(scope);
            all_args.push(arg_string);
        }
        error!("{}", all_args.join(" "));

        let returned_value_string = v8::Number::new(scope, 0f64).into();
        rv.set(returned_value_string);
    }
    pub fn debug(
        scope: &mut v8::HandleScope,
        args: v8::FunctionCallbackArguments,
        mut rv: v8::ReturnValue,
    ) {
        let mut all_args: Vec<String> = vec![];
        for i in 0..args.length() {
            let arg = args.get(i);
            let arg_string = arg.to_string(scope).unwrap().to_rust_string_lossy(scope);
            all_args.push(arg_string);
        }
        debug!("{}", all_args.join(" "));

        let returned_value_string = v8::Number::new(scope, 0f64).into();
        rv.set(returned_value_string);
    }
    pub fn log(
        scope: &mut v8::HandleScope,
        args: v8::FunctionCallbackArguments,
        mut rv: v8::ReturnValue,
    ) {
        let mut all_args: Vec<String> = vec![];
        for i in 0..args.length() {
            let arg = args.get(i);
            let arg_string = arg.to_string(scope).unwrap().to_rust_string_lossy(scope);
            all_args.push(arg_string);
        }
        println!("{}", all_args.join(" "));

        let returned_value_string = v8::Number::new(scope, 0f64).into();
        rv.set(returned_value_string);
    }
}
