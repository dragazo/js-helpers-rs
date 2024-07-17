# Overview

When writing wasm code in Rust, we often need to delve into the javascript world for some aspects.
With popular crates such as `wasm-bindgen`, `web-sys`, and `js-sys`, it is possible to do all of this in Rust.
However, the code to do so can quickly become comically long.
For instance, the following is the current recommended way to set a javascript global variable from Rust:

```rust
Reflect::set(
    &JsValue::from(web_sys::window().unwrap()),
    &JsValue::from("foo"),
    &JsValue::from("bar")
).unwrap()
```

The goal of this crate is to take all of these large annoying patterns and wrap them in short macro syntax.
For instance, the following code using this crate is equivalent:

```rust
js!(window.foo = "bar").unwrap();
```

The `js!` macro provided by this crate aims to provide as close to the full syntax of javascript as possible and unfold into normal Rust code using `wasm-bindgen`, `web-sys`, and `js-sys`.
Importantly, the `js!` macro does not simply `eval` its content, and instead generates normal Rust code that is automatically syntax and type checked at compile time.

The following is a breakdown of the supported syntax features; note that all of these features may be combined arbitrarily in any order.

## Value Creation

You can create any standard javascript value type using normal javascript syntax.
This includes numbers, strings, booleans, arrays, and objects:

```rust
let my_num = js!(45).unwrap();
let my_str = js!("hello world").unwrap();
let my_arr = js!([ 1, true, null, undefined, "test" ]).unwrap();
let my_obj = js!({ name: "john", "with space": true, my_arr }).unwrap();
```

## Variable Access

You can get or assign values to javascript variables/fields/etc. using normal javascript syntax.
Assignment expressions evaluate to the assigned value, just as in normal javascript.
To set global variables, you can access them from the `window` object (see above).

```rust
let my_obj = js!({ name: "john", "with space": true, my_arr: [1, 2, 3] }).unwrap();
js!(my_obj.name = "kevin").unwrap();
js!(my_obj["with space"] = 17).unwrap();
js!(my_obj.my_arr[1] = { hello: true, world: false }).unwrap();
```

As seen above, `js!` supports both the dot (`.`) and bracket (`[]`) syntax options for accessing objects.
Additionally, the nullable dot notation (`?.`) is also supported with the same semantics as in javascript:

```rust
let val = js!(my_obj?.foo?.bar?.baz).unwrap();
```

## Function and Method Calls

The `js!` macro also supports the ability to call functions and methods.

```rust
let my_obj = ...; // pretend we have an object with functions
js!(my_obj.foo(1, 2, 3)).unwrap();
js!(window.open("http://google.com", "_blank")).unwrap();
```

The syntax `<obj>.<func>(...)` and `<obj>[<func>](...)` denote method calls on the context object `<obj>` while any other function call is treated as a non-method (i.e., no context `this` object).

## Functions

The `js!` macro supports creating anonymous functions via the normal javascript arrow (`=>`) or explicit function (`function`) syntax.

```rust
let f = js!((x, y) => x + y).unwrap();
let g = js!(somethingAsync().then(res => res.body)).unwrap();
let h = js!(function (x, y, z) { return x + y * z; }).unwrap();
```

However, due to current limitations of the wasm dynamic function interface, these created arrow functions are non-capturing and importantly cannot refer to Rust `JsValue` objects in scope.
This is because their bodies are essentially `eval` strings, and therefore do not get the extra Rust features or compile time syntax checking of normal `js!` macro usage.

Despite this, as previously demonstrated you can still call a function with Rust objects as arguments.
This provides a means of simulating capturing closures using higher order functions.

```rust
let my_obj = js!({ hello: true, world: 56 }).unwrap();
let f = js!((my_obj => (x, y) => x + y + my_obj.world)(my_obj)).unwrap();
```

## Special Tokens

As already seen, you can access Rust `JsValue` objects by name within a `js!` macro invocation.
However, the following identifiers are reserved by the macro syntax and have special meaning:

- `null` - gets an instance of the javascript `null` value
- `undefined` - gets an instance of the javascript `undefined` value
- `window` - gets a reference to the global window object

## Arbitrary Combinations

As mentioned before, all of these features may be combined arbitrarily in any order.
As an example, you could do something like the following:

```rust
js!(window.username = window.sessions[window.sessionId].getState().username).unwrap();
js!(window.console.log("status", {"active": "on", "inactive": "off"}[window.status])).unwrap();
```

# Development

When developing this crate, use the following command to run all tests:

```sh
wasm-pack test --chrome
```
