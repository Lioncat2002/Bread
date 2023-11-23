use gtk::{prelude::*, ButtonsType, DialogFlags, MessageDialog, MessageType, Window};

pub fn alert(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    let arg = args.get(0);
    let arg_string = arg.to_string(scope).unwrap().to_rust_string_lossy(scope);
    //println!("{}", arg_string);

    MessageDialog::new(
        None::<&Window>,
        DialogFlags::empty(),
        MessageType::Info,
        ButtonsType::Ok,
        &arg_string,
    )
    .run();

    let returned_value_string = v8::Number::new(scope, 0f64).into();
    rv.set(returned_value_string);
}

pub fn log(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    let arg = args.get(0);
    let arg_string = arg.to_string(scope).unwrap().to_rust_string_lossy(scope);
    println!("{}", arg_string);

    let returned_value_string = v8::Number::new(scope, 0f64).into();
    rv.set(returned_value_string);
}
