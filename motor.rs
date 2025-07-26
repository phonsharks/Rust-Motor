use std::env;
use std::fs;
use v8::{self, Context, HandleScope, Local, OwnedIsolate, Platform, Script, String as V8String, Value};

fn main() {
    //v8 platformu oluşturuyoruz.
    //We are creating a v8 platform.
    let platform = v8::new_default_platform(0, false).make_shared();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();
    {
        //isolate js için virtual machine gibi düşünülebilir.
        //isolate js can be thought of as a virtual machine.
        let isolate = &mut v8::Isolate::new(v8::CreateParams::default());
        

        //memory leak işlemini engelliyoruz burada.
        //We are preventing memory leaks here.
        let handle_scope = &mut v8::HandleScope::new(isolate);

        //context js için global scope gibi düşünülebilir.
         //context can be thought of as the global scope for js.
        let context = v8::Context::new(handle_scope);

        //scope js için local scope gibi düşünülebilir.
         //scope can be thought of as similar to local scope in JavaScript.
        let scope = &mut v8::ContextScope::new(handle_scope, context);

        //script kodunu yazıyoruz.
         //We are writing the script code.
        let script : &str = r#"
            function sayHello(name){
                return `Hello, ${name}!`;
            }

            sayHello('Rust');
            "#;
        //args ile dosya yolunu alıyoruz.
         //We get the file path with args.
        let args: Vec<String>=env::args().collect();

        // **
        let file : String =fs::read_to_string(&args[1]).unwrap();

        //dosya yolunu stringe çeviriyoruz.
         //Converting the file path to a string.
        let script: &str = file.as_str();

        //script kodunu stringe çeviriyoruz.
        //We are converting the script code to a string.
        let source: Local <V8String> = v8::String::new(scope, script).unwrap();

        //script kodunu compile ediyoruz.
        //We are compiling the script code.
        let script: Local<Script> = v8::Script::compile(scope, source, None).unwrap();

        //script kodunu çalıştırıyoruz.
         //We are running the script code.
        let result: Local<Value> = script.run(scope).unwrap();

        //script kodunu stringe çeviriyoruz.
        //We are converting the script code to a string.
        let result:String = result.to_rust_string_lossy(scope);
        println!("{}", result);
    }

    unsafe {
        v8::V8::dispose();
    }
}
