Rust and V8 JavaScript Engine Integration

This project aims to integrate Google's V8 JavaScript engine with the Rust programming language, enabling dynamic execution of JavaScript code within the Rust environment.

The goal is to leverage Rust's high-performance and secure infrastructure to read JavaScript code from external files and execute it on the V8 engine.

Features
*V8 Engine Integration: The V8 engine is launched and managed within Rust code.
*Dynamic JavaScript Execution: The desired JavaScript file is read and executed using a file path provided from the command line.
*Output Display: The output of the JavaScript code is printed to the screen by Rust.
*Memory Safety: Thanks to Rust's ownership and scope management, it provides a secure structure against memory leaks.

Use Cases
*An example infrastructure for those who want to dynamically execute JavaScript code in a Rust-based application.
*For those who want to quickly prototype certain functions in JavaScript while doing system programming with Rust.
*A reference project for developers who want to bridge Rust and JavaScript.

How does it work?
*V8 Platform is launched: Rust code launches the V8 engine and prepares the necessary environment.
*JavaScript file is read: The relevant JavaScript file is read using the file path provided from the command line.
*Code is executed: The read JavaScript code is compiled and executed on the V8 engine.
*Result Displayed: The output of the JavaScript code is printed to the screen by Rust.
