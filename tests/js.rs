use wasm_bindgen_test::*;
use js_helpers::js;

#[wasm_bindgen_test]
fn test_null() {
    let _v = js!(null);
    let v: js_helpers::JsMacroResult = js!(null);
    assert_eq!(v.unwrap(), wasm_bindgen::JsValue::NULL);
}

#[wasm_bindgen_test]
fn test_undefined() {
    let _v = js!(undefined);
    let v: js_helpers::JsMacroResult = js!(undefined);
    assert_eq!(v.unwrap(), wasm_bindgen::JsValue::UNDEFINED);
}

#[wasm_bindgen_test]
fn test_bool() {
    let v = js!(true).unwrap();
    assert_eq!(v, wasm_bindgen::JsValue::TRUE);
    assert_ne!(v, wasm_bindgen::JsValue::FALSE);

    let v: wasm_bindgen::JsValue = js!(false).unwrap();
    assert_eq!(v, wasm_bindgen::JsValue::FALSE);
    assert_ne!(v, wasm_bindgen::JsValue::TRUE);
}

#[wasm_bindgen_test]
fn test_numbers() {
    let v = js!(0usize).unwrap();
    assert_eq!(v.as_f64().unwrap(), 0.0);
    let v: wasm_bindgen::JsValue = js!(12i32).unwrap();
    assert_eq!(v.as_f64().unwrap(), 12.0);
    let v: wasm_bindgen::JsValue = js!(-6.5).unwrap();
    assert_eq!(v.as_f64().unwrap(), -6.5);
}

#[wasm_bindgen_test]
fn test_string() {
    let v = js!("hello").unwrap();
    assert_eq!(v.as_string().unwrap(), "hello");
    let v = js!(String::from("world")).unwrap();
    assert_eq!(v.as_string().unwrap(), "world");
}

#[wasm_bindgen_test]
fn test_array() {
    let _v1 = js!([]);
    let v1s: wasm_bindgen::JsValue = js!([]).unwrap();
    let v: &web_sys::js_sys::Array = wasm_bindgen::JsCast::dyn_ref(&v1s).unwrap();
    assert_eq!(v.length(), 0);
    let v1 = js!([12 + 7, true && false, false || true, null, undefined, [], [34 - 1, "mer", v1s, v1s, {age: 7-2},]]).unwrap();
    let v: &web_sys::js_sys::Array = wasm_bindgen::JsCast::dyn_ref(&v1).unwrap();
    assert_eq!(v.length(), 7);
    assert_eq!(v.get(0).as_f64().unwrap(), 19.0);
    assert_eq!(v.get(1).as_bool().unwrap(), false);
    assert_eq!(v.get(2).as_bool().unwrap(), true);
    assert_eq!(v.get(3), wasm_bindgen::JsValue::NULL);
    assert_eq!(v.get(4), wasm_bindgen::JsValue::UNDEFINED);
    let v_5 = wasm_bindgen::JsCast::dyn_into::<web_sys::js_sys::Array>(v.get(5)).unwrap();
    assert_eq!(v_5.length(), 0);
    let v_6 = wasm_bindgen::JsCast::dyn_into::<web_sys::js_sys::Array>(v.get(6)).unwrap();
    assert_eq!(v_6.length(), 5);
    assert_eq!(v_6.get(0).as_f64().unwrap(), 33.0);
    assert_eq!(v_6.get(1).as_string().unwrap(), "mer");
    assert_eq!(v_6.get(2), v_6.get(3));
    assert_eq!(v_6.get(2), v1s);
    assert_ne!(v_6.get(2), js!([]).unwrap());
    assert_ne!(v_6.get(4), js!({}).unwrap());
    assert_eq!(web_sys::js_sys::Reflect::get(&v_6.get(4), &"age".into()).unwrap().as_f64().unwrap(), 5.0);
}

#[wasm_bindgen_test]
fn test_object() {
    let v1 = js!({}).unwrap();
    let v2 = js!({ hello: null, more: undefined, another: 4 + 6, names: ["adam", "john"], meta: {age: 6+2, index: v1} }).unwrap();
    assert_eq!(web_sys::js_sys::Reflect::own_keys(&v2).unwrap().length(), 5);
    assert_eq!(web_sys::js_sys::Reflect::get(&v2, &"hello".into()).unwrap(), wasm_bindgen::JsValue::NULL);
    assert_eq!(web_sys::js_sys::Reflect::get(&v2, &"more".into()).unwrap(), wasm_bindgen::JsValue::UNDEFINED);
    assert_eq!(web_sys::js_sys::Reflect::get(&v2, &"another".into()).unwrap().as_f64().unwrap(), 10.0);
    assert_eq!(web_sys::js_sys::Reflect::get(&v2, &"names".into()).unwrap().is_array(), true);
    let v_names = wasm_bindgen::JsCast::dyn_into::<web_sys::js_sys::Array>(web_sys::js_sys::Reflect::get(&v2, &"names".into()).unwrap()).unwrap();
    assert_eq!(v_names.length(), 2);
    assert_eq!(v_names.get(0).as_string().unwrap(), "adam");
    assert_eq!(v_names.get(1).as_string().unwrap(), "john");
    assert_eq!(web_sys::js_sys::Reflect::get(&v2, &"meta".into()).unwrap().is_object(), true);
    let v_meta = web_sys::js_sys::Reflect::get(&v2, &"meta".into()).unwrap();
    assert_eq!(web_sys::js_sys::Reflect::own_keys(&v_meta).unwrap().length(), 2);
    assert_eq!(web_sys::js_sys::Reflect::get(&v_meta, &"age".into()).unwrap().as_f64().unwrap(), 8.0);
    assert_eq!(web_sys::js_sys::Reflect::get(&v_meta, &"index".into()).unwrap(), v1);
}

#[wasm_bindgen_test]
fn test_array_access() {
    let v = js!([
        1,
        5,
        [4],
        [[3, 6]],
    ]).unwrap();
    assert_eq!(js!(v[0]).unwrap().as_f64().unwrap(), 1.0);
    assert_eq!(js!(v[1]).unwrap().as_f64().unwrap(), 5.0);
    assert_eq!(js!(v[2][0]).unwrap().as_f64().unwrap(), 4.0);
    assert_eq!(js!(v[3][0][0]).unwrap().as_f64().unwrap(), 3.0);
    assert_eq!(js!(v[3][0][1]).unwrap().as_f64().unwrap(), 6.0);
    assert_eq!(js!(v[3][0][2]).unwrap(), wasm_bindgen::JsValue::UNDEFINED);
    assert_eq!(js!(v[3][1]).unwrap(), wasm_bindgen::JsValue::UNDEFINED);
    assert_eq!(js!(v[4]).unwrap(), wasm_bindgen::JsValue::UNDEFINED);
    match js!(v[4][17]).unwrap_err() {
        js_helpers::JsMacroError::IndexLookup { object: _, index } => assert_eq!(index, 17),
        x => panic!("{x:?}"),
    }
    assert_eq!(js!(v[v[3][0][0]][0][1]).unwrap().as_f64().unwrap(), 6.0);
    assert_eq!(js!(v[v[3][0][0]]["0"][1]).unwrap().as_f64().unwrap(), 6.0);
    assert_eq!(js!(v[v["3"][0][0]]["0"][1]).unwrap().as_f64().unwrap(), 6.0);
    assert_eq!(js!(v[v["3"]["0"]["0"]]["0"]["1"]).unwrap().as_f64().unwrap(), 6.0);
}

#[wasm_bindgen_test]
fn test_object_access() {
    let v = js!({
        hello: 45,
        world: { more: 22 },
    }).unwrap();
    assert_eq!(js!(v.hello).unwrap().as_f64().unwrap(), 45.0);
    assert_eq!(js!(v.world.more).unwrap().as_f64().unwrap(), 22.0);
    assert_eq!(js!(v.foo).unwrap(), wasm_bindgen::JsValue::UNDEFINED);
    match js!(v.foo.bar).unwrap_err() {
        js_helpers::JsMacroError::NameLookup { object: _, name } => assert_eq!(name, "bar"),
        x => panic!("{x:?}"),
    }
    assert_eq!(js!(v.foo?.bar).unwrap(), wasm_bindgen::JsValue::UNDEFINED);
    match js!(v.foo?.bar.baz).unwrap_err() {
        js_helpers::JsMacroError::NameLookup { object: _, name } => assert_eq!(name, "baz"),
        x => panic!("{x:?}"),
    }
    assert_eq!(js!(v.foo?.bar?.baz).unwrap(), wasm_bindgen::JsValue::UNDEFINED);
}
