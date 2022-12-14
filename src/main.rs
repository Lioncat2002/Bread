use gtk::{prelude::*, ButtonsType, DialogFlags, MessageDialog, MessageType, Window};
fn main() {
    let platform = v8::new_default_platform(0, false).make_shared();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    let isolate = &mut v8::Isolate::new(Default::default());
    let scope = &mut v8::HandleScope::new(isolate);
    let context = v8::Context::new(scope);
    let scope = &mut v8::ContextScope::new(scope, context);

    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    //This function will be exposed to calling from javascript and will be on global object
    let first_function = v8::Function::new(
        scope,
        |scope: &mut v8::HandleScope,
         args: v8::FunctionCallbackArguments,
         mut rv: v8::ReturnValue| {
            let arg = args.get(0);
            let arg_string = arg.to_string(scope).unwrap().to_rust_string_lossy(scope);
            println!("{}", arg_string);

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
        },
    )
    .unwrap()
    .into();

    //Name of function which be used in javascript
    let name = v8::String::new(scope, "alert").unwrap().into();

    //Global javascript object
    let global = context.global(scope);

    //Set my function to global javascript object
    global.set(scope, name, first_function);

    let code = v8::String::new(scope, "alert('This message is from javascript')").unwrap();
    let script = v8::Script::compile(scope, code, None).unwrap();
    let result = script.run(scope).unwrap().to_rust_string_lossy(scope);
    println!("result: {}", result);
}
