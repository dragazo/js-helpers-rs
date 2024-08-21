#![forbid(unsafe_code)]
#![no_implicit_prelude]

::wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[::wasm_bindgen_test::wasm_bindgen_test]
fn test_null() {
    let _v = ::js_helpers::js!(null);
    let v: ::js_helpers::JsMacroResult = ::js_helpers::js!(null);
    ::std::assert_eq!(v.unwrap(), ::wasm_bindgen::JsValue::NULL);
}

#[::wasm_bindgen_test::wasm_bindgen_test]
fn test_undefined() {
    let _v = ::js_helpers::js!(undefined);
    let v: ::js_helpers::JsMacroResult = ::js_helpers::js!(undefined);
    ::std::assert_eq!(v.unwrap(), ::wasm_bindgen::JsValue::UNDEFINED);
}

#[::wasm_bindgen_test::wasm_bindgen_test]
fn test_bool() {
    let v = ::js_helpers::js!(true).unwrap();
    ::std::assert_eq!(v, ::wasm_bindgen::JsValue::TRUE);
    ::std::assert_ne!(v, ::wasm_bindgen::JsValue::FALSE);

    let v: ::wasm_bindgen::JsValue = ::js_helpers::js!(false).unwrap();
    ::std::assert_eq!(v, ::wasm_bindgen::JsValue::FALSE);
    ::std::assert_ne!(v, ::wasm_bindgen::JsValue::TRUE);
}

#[::wasm_bindgen_test::wasm_bindgen_test]
fn test_numbers() {
    let v = ::js_helpers::js!(0usize).unwrap();
    ::std::assert_eq!(v.as_f64().unwrap(), 0.0);
    let v: ::wasm_bindgen::JsValue = ::js_helpers::js!(12i32).unwrap();
    ::std::assert_eq!(v.as_f64().unwrap(), 12.0);
    let v: ::wasm_bindgen::JsValue = ::js_helpers::js!(-6.5).unwrap();
    ::std::assert_eq!(v.as_f64().unwrap(), -6.5);
}

#[::wasm_bindgen_test::wasm_bindgen_test]
fn test_string() {
    let v = ::js_helpers::js!("hello").unwrap();
    ::std::assert_eq!(v.as_string().unwrap(), "hello");
    let v = ::js_helpers::js!(<::std::string::String as ::std::convert::From<_>>::from("world")).unwrap();
    ::std::assert_eq!(v.as_string().unwrap(), "world");
}

#[::wasm_bindgen_test::wasm_bindgen_test]
fn test_array() {
    let _v1 = ::js_helpers::js!([]);
    let v1s: ::wasm_bindgen::JsValue = ::js_helpers::js!([]).unwrap();
    let v: &::web_sys::js_sys::Array = ::wasm_bindgen::JsCast::dyn_ref(&v1s).unwrap();
    ::std::assert_eq!(v.length(), 0);
    let v1 = ::js_helpers::js!([12 + 7, true && false, false || true, null, undefined, [], [34 - 1, "mer", v1s, v1s, {age: 7-2},]]).unwrap();
    let v: &::web_sys::js_sys::Array = ::wasm_bindgen::JsCast::dyn_ref(&v1).unwrap();
    ::std::assert_eq!(v.length(), 7);
    ::std::assert_eq!(v.get(0).as_f64().unwrap(), 19.0);
    ::std::assert_eq!(v.get(1).as_bool().unwrap(), false);
    ::std::assert_eq!(v.get(2).as_bool().unwrap(), true);
    ::std::assert_eq!(v.get(3), ::wasm_bindgen::JsValue::NULL);
    ::std::assert_eq!(v.get(4), ::wasm_bindgen::JsValue::UNDEFINED);
    let v_5 = ::wasm_bindgen::JsCast::dyn_into::<::web_sys::js_sys::Array>(v.get(5)).unwrap();
    ::std::assert_eq!(v_5.length(), 0);
    let v_6 = ::wasm_bindgen::JsCast::dyn_into::<::web_sys::js_sys::Array>(v.get(6)).unwrap();
    ::std::assert_eq!(v_6.length(), 5);
    ::std::assert_eq!(v_6.get(0).as_f64().unwrap(), 33.0);
    ::std::assert_eq!(v_6.get(1).as_string().unwrap(), "mer");
    ::std::assert_eq!(v_6.get(2), v_6.get(3));
    ::std::assert_eq!(v_6.get(2), v1s);
    ::std::assert_ne!(v_6.get(2), ::js_helpers::js!([]).unwrap());
    ::std::assert_ne!(v_6.get(4), ::js_helpers::js!({}).unwrap());
    ::std::assert_eq!(::web_sys::js_sys::Reflect::get(&v_6.get(4), &::std::convert::Into::into("age")).unwrap().as_f64().unwrap(), 5.0);
}

#[::wasm_bindgen_test::wasm_bindgen_test]
fn test_object() {
    let v1 = ::js_helpers::js!({}).unwrap();
    let v2 = ::js_helpers::js!({ hello: null, more: undefined, another: 4 + 6, names: ["adam", "john"], meta: {age: 6+2, index: v1} }).unwrap();
    ::std::assert_eq!(::web_sys::js_sys::Reflect::own_keys(&v2).unwrap().length(), 5);
    ::std::assert_eq!(::web_sys::js_sys::Reflect::get(&v2, &::std::convert::Into::into("hello")).unwrap(), ::wasm_bindgen::JsValue::NULL);
    ::std::assert_eq!(::web_sys::js_sys::Reflect::get(&v2, &::std::convert::Into::into("more")).unwrap(), ::wasm_bindgen::JsValue::UNDEFINED);
    ::std::assert_eq!(::web_sys::js_sys::Reflect::get(&v2, &::std::convert::Into::into("another")).unwrap().as_f64().unwrap(), 10.0);
    ::std::assert_eq!(::web_sys::js_sys::Reflect::get(&v2, &::std::convert::Into::into("names")).unwrap().is_array(), true);
    let v_names = ::wasm_bindgen::JsCast::dyn_into::<::web_sys::js_sys::Array>(::web_sys::js_sys::Reflect::get(&v2, &::std::convert::Into::into("names")).unwrap()).unwrap();
    ::std::assert_eq!(v_names.length(), 2);
    ::std::assert_eq!(v_names.get(0).as_string().unwrap(), "adam");
    ::std::assert_eq!(v_names.get(1).as_string().unwrap(), "john");
    ::std::assert_eq!(::web_sys::js_sys::Reflect::get(&v2, &::std::convert::Into::into("meta")).unwrap().is_object(), true);
    let v_meta = ::web_sys::js_sys::Reflect::get(&v2, &::std::convert::Into::into("meta")).unwrap();
    ::std::assert_eq!(::web_sys::js_sys::Reflect::own_keys(&v_meta).unwrap().length(), 2);
    ::std::assert_eq!(::web_sys::js_sys::Reflect::get(&v_meta, &::std::convert::Into::into("age")).unwrap().as_f64().unwrap(), 8.0);
    ::std::assert_eq!(::web_sys::js_sys::Reflect::get(&v_meta, &::std::convert::Into::into("index")).unwrap(), v1);
}

#[::wasm_bindgen_test::wasm_bindgen_test]
fn test_array_access() {
    let v = ::js_helpers::js!([
        1,
        5,
        [4],
        [[3, 6]],
    ]).unwrap();
    ::std::assert_eq!(::js_helpers::js!(v[0]).unwrap().as_f64().unwrap(), 1.0);
    ::std::assert_eq!(::js_helpers::js!(v[1]).unwrap().as_f64().unwrap(), 5.0);
    ::std::assert_eq!(::js_helpers::js!(v[2][0]).unwrap().as_f64().unwrap(), 4.0);
    ::std::assert_eq!(::js_helpers::js!(v[3][0][0]).unwrap().as_f64().unwrap(), 3.0);
    ::std::assert_eq!(::js_helpers::js!(v[3][0][1]).unwrap().as_f64().unwrap(), 6.0);
    ::std::assert_eq!(::js_helpers::js!(v[3][0][2]).unwrap(), ::wasm_bindgen::JsValue::UNDEFINED);
    ::std::assert_eq!(::js_helpers::js!(v[3][1]).unwrap(), ::wasm_bindgen::JsValue::UNDEFINED);
    ::std::assert_eq!(::js_helpers::js!(v[4]).unwrap(), ::wasm_bindgen::JsValue::UNDEFINED);
    match ::js_helpers::js!(v[4][17]).unwrap_err() {
        ::js_helpers::JsMacroError::Lookup { object: _, index } => ::std::assert_eq!(index, 17),
        x => ::std::panic!("{x:?}"),
    }
    ::std::assert_eq!(::js_helpers::js!(v[v[3][0][0]][0][1]).unwrap().as_f64().unwrap(), 6.0);
    ::std::assert_eq!(::js_helpers::js!(v[v[3][0][0]]["0"][1]).unwrap().as_f64().unwrap(), 6.0);
    ::std::assert_eq!(::js_helpers::js!(v[v["3"][0][0]]["0"][1]).unwrap().as_f64().unwrap(), 6.0);
    ::std::assert_eq!(::js_helpers::js!(v[v["3"]["0"]["0"]]["0"]["1"]).unwrap().as_f64().unwrap(), 6.0);
    ::std::assert_eq!(::js_helpers::js!((v[v["3"]["0"]["0"]]["0"]["1"])).unwrap().as_f64().unwrap(), 6.0);
    ::std::assert_eq!(::js_helpers::js!(((v[v["3"]["0"]["0"]]["0"])["1"])).unwrap().as_f64().unwrap(), 6.0);
    ::std::assert_eq!(::js_helpers::js!(((v[(v["3"])["0"]["0"]]["0"])["1"])).unwrap().as_f64().unwrap(), 6.0);
}

#[::wasm_bindgen_test::wasm_bindgen_test]
fn test_object_access() {
    let v = ::js_helpers::js!({
        hello: 45,
        world: { more: 22 },
    }).unwrap();
    ::std::assert_eq!(::js_helpers::js!(v.hello).unwrap().as_f64().unwrap(), 45.0);
    ::std::assert_eq!(::js_helpers::js!(v["hello"]).unwrap().as_f64().unwrap(), 45.0);
    ::std::assert_eq!(::js_helpers::js!(v[("hello")]).unwrap().as_f64().unwrap(), 45.0);
    ::std::assert_eq!(::js_helpers::js!(v.world.more).unwrap().as_f64().unwrap(), 22.0);
    ::std::assert_eq!(::js_helpers::js!(v["world"].more).unwrap().as_f64().unwrap(), 22.0);
    ::std::assert_eq!(::js_helpers::js!(v["world"]["more"]).unwrap().as_f64().unwrap(), 22.0);
    ::std::assert_eq!(::js_helpers::js!(v.foo).unwrap(), ::wasm_bindgen::JsValue::UNDEFINED);
    match ::js_helpers::js!(v.foo.bar).unwrap_err() {
        ::js_helpers::JsMacroError::Lookup { object: _, index } => ::std::assert_eq!(index, "bar"),
        x => ::std::panic!("{x:?}"),
    }
    ::std::assert_eq!(::js_helpers::js!(v.foo?.bar).unwrap(), ::wasm_bindgen::JsValue::UNDEFINED);
    match ::js_helpers::js!(v.foo?.bar.baz).unwrap_err() {
        ::js_helpers::JsMacroError::Lookup { object: _, index } => ::std::assert_eq!(index, "baz"),
        x => ::std::panic!("{x:?}"),
    }
    ::std::assert_eq!(::js_helpers::js!(v.foo?.bar?.baz).unwrap(), ::wasm_bindgen::JsValue::UNDEFINED);
}

#[::wasm_bindgen_test::wasm_bindgen_test]
fn test_function_calls() {
    let add = ::web_sys::js_sys::Function::new_with_args("a, b", "return a + b;");
    let v = ::js_helpers::js!({
        add,
        sub: ::web_sys::js_sys::Function::new_with_args("a, b", "return a - b;"),
        more: {
            stuff: { a: add },
        },
        stuff: ::web_sys::js_sys::Function::new_with_args("a, b", "return { x: a, y: { z: b } };"),
        deeper: ::web_sys::js_sys::Function::new_with_args("a, b", "return (c) => () => a * b + c;")
    }).unwrap();
    ::std::assert_eq!(::js_helpers::js!(add(5, 4)).unwrap().as_f64().unwrap(), 9.0);
    ::std::assert_eq!(::js_helpers::js!(v.add(5, 4)).unwrap().as_f64().unwrap(), 9.0);
    ::std::assert_eq!(::js_helpers::js!(v["add"](5, 4)).unwrap().as_f64().unwrap(), 9.0);
    ::std::assert_eq!(::js_helpers::js!(v.sub(5, 4)).unwrap().as_f64().unwrap(), 1.0);
    ::std::assert_eq!(::js_helpers::js!(v.more.stuff.a(1, 6)).unwrap().as_f64().unwrap(), 7.0);
    ::std::assert_eq!(::js_helpers::js!(v["more"].stuff.a(1, 6)).unwrap().as_f64().unwrap(), 7.0);
    ::std::assert_eq!(::js_helpers::js!(v["more"]["stuff"].a(1, 6)).unwrap().as_f64().unwrap(), 7.0);
    ::std::assert_eq!(::js_helpers::js!(v.stuff(2, 3).x).unwrap().as_f64().unwrap(), 2.0);
    ::std::assert_eq!(::js_helpers::js!(v.stuff(2, 3).y.z).unwrap().as_f64().unwrap(), 3.0);
    ::std::assert_eq!(::js_helpers::js!(v.deeper(6, 7)(2)()).unwrap().as_f64().unwrap(), 44.0);
    ::std::assert_eq!(::js_helpers::js!((v.deeper(6, 7)(2))()).unwrap().as_f64().unwrap(), 44.0);
    ::std::assert_eq!(::js_helpers::js!(((v.deeper(6, 7))(2))()).unwrap().as_f64().unwrap(), 44.0);
    ::std::assert_eq!(::js_helpers::js!((((v.deeper)(6, 7))(2))()).unwrap().as_f64().unwrap(), 44.0);
    ::std::assert_eq!(::js_helpers::js!(((((v).deeper)(6, 7))(2))()).unwrap().as_f64().unwrap(), 44.0);

    let x = ::js_helpers::js!([ v.deeper(6, 7)(2)() ]).unwrap();
    ::std::assert_eq!(::js_helpers::js!(x[0]).unwrap().as_f64().unwrap(), 44.0);

    let x = ::js_helpers::js!({ zz: v.deeper(6, 7)(2)(), "yy": 12, "x x": 7 }).unwrap();
    ::std::assert_eq!(::js_helpers::js!(x.zz).unwrap().as_f64().unwrap(), 44.0);
    ::std::assert_eq!(::js_helpers::js!(x["zz"]).unwrap().as_f64().unwrap(), 44.0);
    ::std::assert_eq!(::js_helpers::js!(x.yy).unwrap().as_f64().unwrap(), 12.0);
    ::std::assert_eq!(::js_helpers::js!(x["yy"]).unwrap().as_f64().unwrap(), 12.0);
    ::std::assert_eq!(::js_helpers::js!(x["x x"]).unwrap().as_f64().unwrap(), 7.0);
}

#[::wasm_bindgen_test::wasm_bindgen_test]
fn test_complex_root() {
    ::std::assert_eq!(::js_helpers::js!([5,true,null]["0"]).unwrap().as_f64().unwrap(), 5.0);
    ::std::assert_eq!(::js_helpers::js!([5,true,null]["1"]).unwrap().as_bool().unwrap(), true);
    ::std::assert_eq!(::js_helpers::js!([5,true,null]["2"]).unwrap().is_null(), true);

    ::std::assert_eq!(::js_helpers::js!({ test: 5, more: false, again: null }.test).unwrap().as_f64().unwrap(), 5.0);
    ::std::assert_eq!(::js_helpers::js!({ test: 5, more: false, again: null }["test"]).unwrap().as_f64().unwrap(), 5.0);
    ::std::assert_eq!(::js_helpers::js!({ test: 5, more: false, again: null }.more).unwrap().as_bool().unwrap(), false);
    ::std::assert_eq!(::js_helpers::js!({ test: 5, more: false, again: null }["more"]).unwrap().as_bool().unwrap(), false);
    ::std::assert_eq!(::js_helpers::js!({ test: 5, more: false, again: null }.again).unwrap().is_null(), true);
    ::std::assert_eq!(::js_helpers::js!({ test: 5, more: false, again: null }["again"]).unwrap().is_null(), true);
}

#[::wasm_bindgen_test::wasm_bindgen_test]
fn test_special_ident() {
    ::std::assert_eq!(::js_helpers::js!(null?.foo).unwrap().is_undefined(), true);
    ::std::assert_eq!(::js_helpers::js!(undefined?.foo).unwrap().is_undefined(), true);

    let w = ::js_helpers::js!(window).unwrap();
    let a1 = ::js_helpers::js!(w.alert).unwrap();
    let a2 = ::js_helpers::js!(window.alert).unwrap();
    ::std::assert_eq!(a1, a2);
}

#[::wasm_bindgen_test::wasm_bindgen_test]
fn test_assignment() {
    let x = ::js_helpers::js!({
        foo: 45,
        bar: 12,
        baz: { mer: 2, der: [3, 4, 2] },
    }).unwrap();

    ::std::assert_eq!(::js_helpers::js!(x.foo).unwrap().as_f64().unwrap(), 45.0);
    ::std::assert_eq!(::js_helpers::js!(x.foo = 20).unwrap().as_f64().unwrap(), 20.0);
    ::std::assert_eq!(::js_helpers::js!(x.foo).unwrap().as_f64().unwrap(), 20.0);
    ::std::assert_eq!(::js_helpers::js!(x["foo"] = 42).unwrap().as_f64().unwrap(), 42.0);
    ::std::assert_eq!(::js_helpers::js!(x.foo).unwrap().as_f64().unwrap(), 42.0);

    ::std::assert_eq!(::js_helpers::js!(x.baz.mer).unwrap().as_f64().unwrap(), 2.0);
    ::std::assert_eq!(::js_helpers::js!(x.baz.mer = 7).unwrap().as_f64().unwrap(), 7.0);
    ::std::assert_eq!(::js_helpers::js!(x.baz.mer).unwrap().as_f64().unwrap(), 7.0);
    ::std::assert_eq!(::js_helpers::js!(x.baz["mer"] = -2).unwrap().as_f64().unwrap(), -2.0);
    ::std::assert_eq!(::js_helpers::js!(x.baz.mer).unwrap().as_f64().unwrap(), -2.0);

    ::std::assert_eq!(::js_helpers::js!(x.baz.der.length).unwrap().as_f64().unwrap(), 3.0);
    ::std::assert_eq!(::js_helpers::js!(x.baz.der[0]).unwrap().as_f64().unwrap(), 3.0);
    ::std::assert_eq!(::js_helpers::js!(x.baz.der[1]).unwrap().as_f64().unwrap(), 4.0);
    ::std::assert_eq!(::js_helpers::js!(x.baz.der[2]).unwrap().as_f64().unwrap(), 2.0);
    ::std::assert_eq!(::js_helpers::js!(x.baz.der[1] = 35).unwrap().as_f64().unwrap(), 35.0);
    ::std::assert_eq!(::js_helpers::js!(x.baz.der.length).unwrap().as_f64().unwrap(), 3.0);
    ::std::assert_eq!(::js_helpers::js!(x.baz.der[0]).unwrap().as_f64().unwrap(), 3.0);
    ::std::assert_eq!(::js_helpers::js!(x.baz.der[1]).unwrap().as_f64().unwrap(), 35.0);
    ::std::assert_eq!(::js_helpers::js!(x.baz.der[2]).unwrap().as_f64().unwrap(), 2.0);
    ::std::assert_eq!(::js_helpers::js!(x.baz.der["4"] = 22).unwrap().as_f64().unwrap(), 22.0);
    ::std::assert_eq!(::js_helpers::js!(x.baz.der.length).unwrap().as_f64().unwrap(), 5.0);
    ::std::assert_eq!(::js_helpers::js!(x.baz.der[0]).unwrap().as_f64().unwrap(), 3.0);
    ::std::assert_eq!(::js_helpers::js!(x.baz.der[1]).unwrap().as_f64().unwrap(), 35.0);
    ::std::assert_eq!(::js_helpers::js!(x.baz.der[2]).unwrap().as_f64().unwrap(), 2.0);
    ::std::assert_eq!(::js_helpers::js!(x.baz.der[3]).unwrap().is_undefined(), true);
    ::std::assert_eq!(::js_helpers::js!(x.baz.der[4]).unwrap().as_f64().unwrap(), 22.0);
}

#[::wasm_bindgen_test::wasm_bindgen_test]
fn test_closures() {
    let f1 = ::js_helpers::js!((a, b) => a + b).unwrap();
    let f2 = ::js_helpers::js!((x) => x * 2).unwrap();
    let f3 = ::js_helpers::js!(z => z - 5).unwrap();
    let f4 = ::js_helpers::js!(z => { return z + 1; }).unwrap();
    let f5 = ::js_helpers::js!((f2 => (z, w) => z * w * f2(w))(f2)).unwrap();
    let f6 = ::js_helpers::js!(function (a, b) { return a * b + 1; }).unwrap();

    ::std::assert_eq!(::js_helpers::js!(f1(4, 5)).unwrap().as_f64().unwrap(), 9.0);
    ::std::assert_eq!(::js_helpers::js!(f2(7)).unwrap().as_f64().unwrap(), 14.0);
    ::std::assert_eq!(::js_helpers::js!(f3(8)).unwrap().as_f64().unwrap(), 3.0);
    ::std::assert_eq!(::js_helpers::js!(f4(11)).unwrap().as_f64().unwrap(), 12.0);
    ::std::assert_eq!(::js_helpers::js!(f5(7, 4)).unwrap().as_f64().unwrap(), 224.0);
    ::std::assert_eq!(::js_helpers::js!(f6(3, 4)).unwrap().as_f64().unwrap(), 13.0);
}

#[::wasm_bindgen_test::wasm_bindgen_test]
fn test_rust_exprs() {
    #[derive(Clone)]
    struct MyString(::std::string::String);
    impl MyString {
        fn as_str(&self) -> &str { &self.0 }
    }

    let f = ::js_helpers::js!((a, b) => a + " " + b).unwrap();
    let g = ::js_helpers::js!((x, y) => x[y]).unwrap();

    let a = "first";
    let b = <::std::string::String as ::std::convert::From<_>>::from("second");
    let c = MyString(<::std::string::String as ::std::convert::From<_>>::from("third"));

    ::std::assert_eq!(::js_helpers::js!(f("zero", "world")).unwrap().as_string().unwrap(), "zero world");
    ::std::assert_eq!(::js_helpers::js!(f(a, "world")).unwrap().as_string().unwrap(), "first world");
    ::std::assert_eq!(::js_helpers::js!(f({ (a) }, "world")).unwrap().as_string().unwrap(), "first world");
    ::std::assert_eq!(::js_helpers::js!(f(b, "world")).unwrap().as_string().unwrap(), "second world");
    ::std::assert_eq!(::js_helpers::js!(f({ c.as_str() }, "world")).unwrap().as_string().unwrap(), "third world");

    ::std::assert_eq!(::js_helpers::js!(g({ a }, "a")).unwrap().as_string().unwrap(), "first");
    ::std::assert_eq!(::js_helpers::js!(g({ b }, "b")).unwrap().as_string().unwrap(), "second");
    ::std::assert_eq!(::js_helpers::js!(g({ c: { c.as_str() } }, "c")).unwrap().as_string().unwrap(), "third");
}
