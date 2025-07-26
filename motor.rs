use std::env;
use std::fs;
use v8::{self, Context, HandleScope, Local, OwnedIsolate, Platform, Script, String as V8String, Value};

fn main() {
    //v8 platformu oluşturuyoruz.
    let platform = v8::new_default_platform(0, false).make_shared();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();
    {
        //isolate js için virtual machine gibi düşünülebilir.
        let isolate = &mut v8::Isolate::new(v8::CreateParams::default());
        

        //memory leak işlemini engelliyoruz burada
        let handle_scope = &mut v8::HandleScope::new(isolate);

        //context js için global scope gibi düşünülebilir.
        let context = v8::Context::new(handle_scope);

        //scope js için local scope gibi düşünülebilir.
        let scope = &mut v8::ContextScope::new(handle_scope, context);

        //script kodunu yazıyoruz.
        let script : &str = r#"
            function sayHello(name){
                return `Hello, ${name}!`;
            }

            sayHello('Rust');
            "#;
        //args ile dosya yolunu alıyoruz.
        let args: Vec<String>=env::args().collect();

        //
        let file : String =fs::read_to_string(&args[1]).unwrap();

        //dosya yolunu stringe çeviriyoruz.
        let script: &str = file.as_str();

        //script kodunu stringe çeviriyoruz.
        let source: Local <V8String> = v8::String::new(scope, script).unwrap();

        //script kodunu compile ediyoruz.
        let script: Local<Script> = v8::Script::compile(scope, source, None).unwrap();

        //script kodunu çalıştırıyoruz.
        let result: Local<Value> = script.run(scope).unwrap();

        //script kodunu stringe çeviriyoruz.
        let result:String = result.to_rust_string_lossy(scope);
        println!("{}", result);
    }

    unsafe {
        v8::V8::dispose();
    }
}