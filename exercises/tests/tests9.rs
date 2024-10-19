// tests9.rs
//
// Rust is highly capable of sharing FFI interfaces with C/C++ and other statically compiled
// languages, and it can even link within the code itself! It makes it through the extern
// block, just like the code below.
//
// The short string after the `extern` keyword indicates which ABI the externally imported
// function would follow. In this exercise, "Rust" is used, while other variants exists like
// "C" for standard C ABI, "stdcall" for the Windows ABI.
//
// The externally imported functions are declared in the extern blocks, with a semicolon to
// mark the end of signature instead of curly braces. Some attributes can be applied to those
// function declarations to modify the linking behavior, such as #[link_name = ".."] to
// modify the actual symbol names.
//
// If you want to export your symbol to the linking environment, the `extern` keyword can
// also be marked before a function definition with the same ABI string note. The default ABI
// for Rust functions is literally "Rust", so if you want to link against pure Rust functions,
// the whole extern term can be omitted.
//
// Rust mangles symbols by default, just like C++ does. To suppress this behavior and make
// those functions addressable by name, the attribute #[no_mangle] can be applied.
//
// In this exercise, your task is to make the testcase able to call the `my_demo_function` in
// module Foo. the `my_demo_function_alias` is an alias for `my_demo_function`, so the two
// line of code in the testcase should call the same function.
//
// You should NOT modify any existing code except for adding two lines of attributes.


// 定义外部函数
extern "Rust" {
    fn my_demo_function(a: u32) -> u32;
    fn my_demo_function_alias(a: u32) -> u32;
}

// 模块 Foo 中的函数
mod Foo {
    // 使用 #[no_mangle] 禁止符号重命名
    // 使用 #[link_name] 指定符号名称
    #[no_mangle]
    pub fn my_demo_function(a: u32) -> u32 {
        a
    }
}

// 在 mod Foo 外部，将 Foo::my_demo_function 作为 my_demo_function 和 my_demo_function_alias
#[link_name = "my_demo_function"]
static MY_DEMO_FUNCTION: unsafe extern "Rust" fn(u32) -> u32 = Foo::my_demo_function;

#[link_name = "my_demo_function"]
static MY_DEMO_FUNCTION_ALIAS: unsafe extern "Rust" fn(u32) -> u32 = Foo::my_demo_function;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        // 使用静态变量作为外部函数的别名
        unsafe {
            assert_eq!(MY_DEMO_FUNCTION(123), 123);
            assert_eq!(MY_DEMO_FUNCTION_ALIAS(456), 456);
        }
    }
}