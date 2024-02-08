mod console;
use std::fs;

fn main() {
    env_logger::init();
    let platform = v8::new_default_platform(0, false).make_shared();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    let isolate = &mut v8::Isolate::new(Default::default());
    let scope = &mut v8::HandleScope::new(isolate);
    let context = v8::Context::new(scope);
    let scope = &mut v8::ContextScope::new(scope, context);

    //Console.log function
    let log_fn = v8::Function::new(scope, console::Console::log)
        .unwrap()
        .into();
    let log_name = v8::String::new(scope, "log").unwrap().into();
    //Console.debug function
    let debug_fn = v8::Function::new(scope, console::Console::debug)
        .unwrap()
        .into();
    let debug_name = v8::String::new(scope, "debug").unwrap().into();
    //Console.error function
    let error_fn = v8::Function::new(scope, console::Console::error)
        .unwrap()
        .into();
    let error_name = v8::String::new(scope, "error").unwrap().into();
    //Console.info function
    let info_fn = v8::Function::new(scope, console::Console::info)
        .unwrap()
        .into();
    let info_name = v8::String::new(scope, "info").unwrap().into();
    //Global javascript object
    let global = context.global(scope);
    //global console object
    let console = context.global(scope);
    let console_name = v8::String::new(scope, "console").unwrap().into();
    //add log function to console
    console.set(scope, log_name, log_fn);
    console.set(scope, debug_name, debug_fn);
    console.set(scope, error_name, error_fn);
    console.set(scope, info_name, info_fn);
    //add console object to the global js object
    global.set(scope, console_name, console.into());

    let src = fs::read_to_string("tests/test.js").unwrap();

    let code = v8::String::new(scope, &src).unwrap();
    let script = v8::Script::compile(scope, code, None).unwrap();
    let return_val = script.run(scope).unwrap().to_rust_string_lossy(scope);
    println!("{}", return_val);
}
