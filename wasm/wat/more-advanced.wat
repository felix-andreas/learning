(module
    (import "js" "memory" (memory 1))
    (import "console" "log" (func $log (param i32) (param i32)))
    (func $getAnswer (export "getAnswer") (result i32)
        i32.const 21
        i32.const 21
        call $add
    )
    (func $add (export "add") (param $num1 i32) (param $num2 i32) (result i32)
        local.get $num1
        local.get $num2
        i32.add
    )
    (data (i32.const 0) "Hello from WebAssembly!")
    (func $hello (export "hello")
        i32.const 0
        i32.const 23
        call $log
    )
)
