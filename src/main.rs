mod console;
use std::fs;

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

    //Alert function
    let alert_fn = v8::Function::new(scope, console::alert).unwrap().into();
    let alert_name = v8::String::new(scope, "alert").unwrap().into();

    //Cosnole.log function
    let log_fn = v8::Function::new(scope, console::log).unwrap().into();
    let log_name = v8::String::new(scope, "log").unwrap().into();
    //Global javascript object

    let global = context.global(scope);
    let console=context.global(scope);
    let console_name = v8::String::new(scope, "console").unwrap().into();
    
    //Set my function to global javascript object
    console.set(scope, alert_name, alert_fn);
    console.set(scope, log_name, log_fn);

    global.set(scope, console_name, console.into());

    let src = fs::read_to_string("tests/test.js").unwrap();

    let code = v8::String::new(scope, &src).unwrap();
    let script = v8::Script::compile(scope, code, None).unwrap();
    let return_val = script.run(scope).unwrap().to_rust_string_lossy(scope);
    println!("return: {}", return_val);
}
