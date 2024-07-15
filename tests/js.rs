use wasm_bindgen_test::*;
use js_helpers::*;

#[wasm_bindgen_test]
fn test_null() {
    let _v = js!(null);
    let v: wasm_bindgen::JsValue = js!(null);
    assert_eq!(v, wasm_bindgen::JsValue::NULL);
}

#[wasm_bindgen_test]
fn test_undefined() {
    let _v = js!(undefined);
    let v: wasm_bindgen::JsValue = js!(undefined);
    assert_eq!(v, wasm_bindgen::JsValue::UNDEFINED);
}

#[wasm_bindgen_test]
fn test_bool() {
    let v = js!(true);
    assert_eq!(v, wasm_bindgen::JsValue::TRUE);
    assert_ne!(v, wasm_bindgen::JsValue::FALSE);

    let v: wasm_bindgen::JsValue = js!(false);
    assert_eq!(v, wasm_bindgen::JsValue::FALSE);
    assert_ne!(v, wasm_bindgen::JsValue::TRUE);
}

#[wasm_bindgen_test]
fn test_numbers() {
    let v = js!(0usize);
    assert_eq!(v.as_f64().unwrap(), 0.0);
    let v: wasm_bindgen::JsValue = js!(12i32);
    assert_eq!(v.as_f64().unwrap(), 12.0);
    let v: wasm_bindgen::JsValue = js!(-6.5);
    assert_eq!(v.as_f64().unwrap(), -6.5);
}

#[wasm_bindgen_test]
fn test_string() {
    let v = js!("hello");
    assert_eq!(v.as_string().unwrap(), "hello");
    let v = js!(String::from("world"));
    assert_eq!(v.as_string().unwrap(), "world");
}

#[wasm_bindgen_test]
fn test_array() {
    let _v1 = js!([]);
    let v1s: wasm_bindgen::JsValue = js!([]);
    let v: &js_sys::Array = wasm_bindgen::JsCast::dyn_ref(&v1s).unwrap();
    assert_eq!(v.length(), 0);
    let v1 = js!([12 + 7, true && false, false || true, null, undefined, [], [34 - 1, "mer", v1s, v1s, {age: 7-2},]]);
    let v: &js_sys::Array = wasm_bindgen::JsCast::dyn_ref(&v1).unwrap();
    assert_eq!(v.length(), 7);
    assert_eq!(v.get(0).as_f64().unwrap(), 19.0);
    assert_eq!(v.get(1).as_bool().unwrap(), false);
    assert_eq!(v.get(2).as_bool().unwrap(), true);
    assert_eq!(v.get(3), wasm_bindgen::JsValue::NULL);
    assert_eq!(v.get(4), wasm_bindgen::JsValue::UNDEFINED);
    let v_5 = wasm_bindgen::JsCast::dyn_into::<js_sys::Array>(v.get(5)).unwrap();
    assert_eq!(v_5.length(), 0);
    let v_6 = wasm_bindgen::JsCast::dyn_into::<js_sys::Array>(v.get(6)).unwrap();
    assert_eq!(v_6.length(), 5);
    assert_eq!(v_6.get(0).as_f64().unwrap(), 33.0);
    assert_eq!(v_6.get(1).as_string().unwrap(), "mer");
    assert_eq!(v_6.get(2), v_6.get(3));
    assert_eq!(v_6.get(2), v1s);
    assert_ne!(v_6.get(2), js!([]));
    assert_ne!(v_6.get(4), js!({}));
    let v_6_4 = wasm_bindgen::JsCast::dyn_into::<js_sys::Map>(v_6.get(4)).unwrap();
    assert_eq!(v_6_4.size(), 1);
    assert_eq!(v_6_4.get(&"age".into()).as_f64().unwrap(), 5.0);
}

#[wasm_bindgen_test]
fn test_object() {
    let v1 = js!({});
    let v = wasm_bindgen::JsCast::dyn_ref::<js_sys::Map>(&v1).unwrap();
    assert_eq!(v.size(), 0);
    let v2 = js!({ hello: null, more: undefined, another: 4 + 6, names: ["adam", "john"], meta: {age: 6+2, index: v1} });
    let v = wasm_bindgen::JsCast::dyn_ref::<js_sys::Map>(&v2).unwrap();
    assert_eq!(v.size(), 5);
    assert_eq!(v.get(&"hello".into()), wasm_bindgen::JsValue::NULL);
    assert_eq!(v.get(&"more".into()), wasm_bindgen::JsValue::UNDEFINED);
    assert_eq!(v.get(&"another".into()).as_f64().unwrap(), 10.0);
    assert_eq!(v.get(&"names".into()).is_array(), true);
    let v_names = wasm_bindgen::JsCast::dyn_into::<js_sys::Array>(v.get(&"names".into())).unwrap();
    assert_eq!(v_names.length(), 2);
    assert_eq!(v_names.get(0).as_string().unwrap(), "adam");
    assert_eq!(v_names.get(1).as_string().unwrap(), "john");
    assert_eq!(v.get(&"meta".into()).is_object(), true);
    let v_meta = wasm_bindgen::JsCast::dyn_into::<js_sys::Map>(v.get(&"meta".into())).unwrap();
    assert_eq!(v_meta.size(), 2);
    assert_eq!(v_meta.get(&"age".into()).as_f64().unwrap(), 8.0);
    assert_eq!(v_meta.get(&"index".into()), v1);
}
