use std::collections::HashMap;

pub fn fetch(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    let resource = args
        .get(0)
        .to_string(scope)
        .unwrap()
        .to_rust_string_lossy(scope);
    let response = reqwest::blocking::get(resource)
        .unwrap()
        .json::<HashMap<String, String>>()
        .unwrap();
    let res_str = serde_json::to_string(&response).unwrap();
    let data = v8::String::new(scope, &res_str).unwrap();
    let res = v8::json::parse(scope, data).unwrap();
    //let returned_value_string = v8::String::new(scope, &res_str).unwrap().into();
    rv.set(res);
}
