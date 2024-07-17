#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

pub use wasm_bindgen;
pub use web_sys::{self, js_sys};

#[derive(Debug)]
pub enum JsMacroError {
    Lookup { object: wasm_bindgen::JsValue, index: wasm_bindgen::JsValue },
    Assign { object: wasm_bindgen::JsValue, index: wasm_bindgen::JsValue },
    NotFunction { object: wasm_bindgen::JsValue },
    FunctionError { object: wasm_bindgen::JsValue, error: wasm_bindgen::JsValue },
    NoWindow,
}
pub type JsMacroResult = Result<wasm_bindgen::JsValue, JsMacroError>;

#[macro_export]
macro_rules! js {
    (null $($rest:tt)*) => {{
        let js_helpers_null = $crate::wasm_bindgen::JsValue::null();
        $crate::js!(js_helpers_null $($rest)*)
    }};
    (undefined $($rest:tt)*) => {{
        let js_helpers_undefined = $crate::wasm_bindgen::JsValue::undefined();
        $crate::js!(js_helpers_undefined $($rest)*)
    }};
    (window $($rest:tt)*) => {
        match $crate::web_sys::window().map($crate::wasm_bindgen::JsValue::from) {
            ::std::option::Option::Some(js_helpers_window) => $crate::js!(js_helpers_window $($rest)*),
            ::std::option::Option::None => $crate::JsMacroResult::Err($crate::JsMacroError::NoWindow),
        }
    };

    // --------------------------------------------------------------------------------------------------------------

    ([ $($t:tt)* ] $($rest:tt)*) => {{
        let mut js_helpers_array_target = $crate::js_sys::Array::new();
        match $crate::js!(@fill_array js_helpers_array_target () $($t)*) {
            $crate::JsMacroResult::Ok(js_helpers_root) => $crate::js!(js_helpers_root $($rest)*),
            x => x,
        }
    }};
    (@fill_array $target:ident ()) => {
        $crate::JsMacroResult::Ok($crate::wasm_bindgen::JsValue::from($target))
    };
    (@fill_array $target:ident ( $($stuff:tt)* ) $(, $($rest:tt)*)?) => {
        match $crate::js!($($stuff)*) {
            $crate::JsMacroResult::Ok(v) => {
                $target.push(&v);
                $crate::js!(@fill_array $target () $($($rest)*)?)
            }
            x => x,
        }
    };
    (@fill_array $target:ident ( $($stuff:tt)* ) $head:tt $($rest:tt)*) => {
        $crate::js!(@fill_array $target ( $($stuff)* $head ) $($rest)*)
    };

    // --------------------------------------------------------------------------------------------------------------

    ({ $($t:tt)* } $($rest:tt)*) => {{
        let mut js_helpers_object_target = $crate::wasm_bindgen::JsValue::from($crate::js_sys::Object::new());
        match $crate::js!(@fill_object js_helpers_object_target () () $($t)*) {
            $crate::JsMacroResult::Ok(js_helpers_root) => $crate::js!(js_helpers_root $($rest)*),
            x => x,
        }
    }};
    (@fill_object $target:ident () ()) => {
        $crate::JsMacroResult::Ok($target)
    };
    (@fill_object $target:ident () () $key:ident $(, $($rest:tt)*)?) => {
        $crate::js!(@fill_object $target () () $key : $key $(, $($rest)*)?)
    };
    (@fill_object $target:ident () () $key:ident : $($rest:tt)*) => {{
        let js_helpers_key_name = $crate::wasm_bindgen::JsValue::from(stringify!($key));
        $crate::js!(@fill_object $target ( js_helpers_key_name ) () $($rest)*)
    }};
    (@fill_object $target:ident () () $key:literal : $($rest:tt)*) => {{
        let js_helpers_key_name = $crate::wasm_bindgen::JsValue::from($key);
        $crate::js!(@fill_object $target ( js_helpers_key_name ) () $($rest)*)
    }};
    (@fill_object $target:ident ( $key:ident ) ( $($stuff:tt)* ) $(, $($rest:tt)*)?) => {
        match $crate::js!($($stuff)*) {
            $crate::JsMacroResult::Ok(v) => {
                $crate::js_sys::Reflect::set(&$target, &$key, &v).unwrap();
                $crate::js!(@fill_object $target () () $($($rest)*)?)
            }
            x => x,
        }
    };
    (@fill_object $target:ident ( $key:ident ) ( $($stuff:tt)* ) $head:tt $($rest:tt)*) => {
        $crate::js!(@fill_object $target ( $key ) ( $($stuff)* $head ) $($rest)*)
    };

    // --------------------------------------------------------------------------------------------------------------

    ($root:ident . $field:ident ( $($args:tt)* ) $($rest:tt)*) => {
        match $crate::js!($root . $field) {
            $crate::JsMacroResult::Ok(js_helpers_function) => $crate::js!(@function_call $root js_helpers_function ( $($args)* ) $($rest)*),
            x => x,
        }
    };
    ($root:ident [ $($index:tt)* ] ( $($args:tt)* ) $($rest:tt)*) => {
        match $crate::js!($root [ $($index)* ]) {
            $crate::JsMacroResult::Ok(js_helpers_function) => $crate::js!(@function_call $root js_helpers_function ( $($args)* ) $($rest)*),
            x => x,
        }
    };
    ($root:ident ( $($args:tt)* ) $($rest:tt)*) => {{
        let js_helpers_context = $crate::wasm_bindgen::JsValue::undefined();
        $crate::js!(@function_call js_helpers_context $root ( $($args)* ) $($rest)*)
    }};
    (@function_call $root:ident $func:ident ( $($args:tt)* ) $($rest:tt)*) => {
        match $crate::wasm_bindgen::JsCast::dyn_ref::<$crate::js_sys::Function>(&$func) {
            ::std::option::Option::Some(js_helpers_function) => match $crate::js!([$($args)*]) {
                $crate::JsMacroResult::Ok(js_helpers_args) => match $crate::js_sys::Reflect::apply(&js_helpers_function, &$root, &$crate::wasm_bindgen::JsCast::dyn_into(js_helpers_args).unwrap()) {
                    ::std::result::Result::Ok(js_helpers_result) => $crate::js!(js_helpers_result $($rest)*),
                    ::std::result::Result::Err(error) => $crate::JsMacroResult::Err($crate::JsMacroError::FunctionError { object: $func.into(), error }),
                }
                x => x,
            }
            ::std::option::Option::None => $crate::JsMacroResult::Err($crate::JsMacroError::NotFunction { object: $func.into() }),
        }
    };

    // --------------------------------------------------------------------------------------------------------------

    ($root:ident . $field:ident = $($rest:tt)*) => {
        match $crate::js!($($rest)*) {
            $crate::JsMacroResult::Ok(x) => match $crate::js_sys::Reflect::set(&$root, &$crate::wasm_bindgen::JsValue::from(stringify!($field)), &x) {
                ::std::result::Result::Ok(true) => $crate::JsMacroResult::Ok(x),
                _ => $crate::JsMacroResult::Err($crate::JsMacroError::Assign { object: $root.clone(), index: stringify!($field).into() }),
            }
            x => x,
        }
    };
    ($root:ident . $field:ident $($rest:tt)*) => {
        match $crate::js_sys::Reflect::get(&$root, &$crate::wasm_bindgen::JsValue::from(stringify!($field))) {
            ::std::result::Result::Ok(js_helpers_sub_object) => $crate::js!(js_helpers_sub_object $($rest)*),
            ::std::result::Result::Err(_) => $crate::JsMacroResult::Err($crate::JsMacroError::Lookup { object: $root.clone(), index: stringify!($field).into() }),
        }
    };
    ($root:ident ?. $field:ident $($rest:tt)*) => {{
        let js_helpers_sub_object = $crate::js_sys::Reflect::get(&$root, &$crate::wasm_bindgen::JsValue::from(stringify!($field))).unwrap_or_else(|_| $crate::wasm_bindgen::JsValue::undefined());
        $crate::js!(js_helpers_sub_object $($rest)*)
    }};

    // --------------------------------------------------------------------------------------------------------------

    ($root:ident [ $($index:tt)* ] = $($rest:tt)*) => {
        match $crate::js!($($index)*) {
            $crate::JsMacroResult::Ok(js_helpers_index) => match $crate::wasm_bindgen::JsCast::dyn_ref::<$crate::js_sys::Array>(&$root) {
                ::std::option::Option::Some(js_helpers_sub_object) => {
                    let js_helpers_index_f64 = js_helpers_index.as_f64().or_else(|| js_helpers_index.as_string().and_then(|x| x.parse::<f64>().ok())).unwrap_or_else(|| 0.5);
                    let js_helpers_index_u32 = js_helpers_index_f64 as u32;
                    match js_helpers_index_f64 == js_helpers_index_u32 as f64 {
                        true => match $crate::js!($($rest)*) {
                            $crate::JsMacroResult::Ok(x) => {
                                js_helpers_sub_object.set(js_helpers_index_u32, x.clone());
                                $crate::JsMacroResult::Ok(x)
                            }
                            x => x,
                        }
                        false => $crate::JsMacroResult::Err($crate::JsMacroError::Assign { object: $root.clone(), index: js_helpers_index }),
                    }
                }
                ::std::option::Option::None => match $crate::wasm_bindgen::JsCast::dyn_ref::<$crate::js_sys::Object>(&$root) {
                    ::std::option::Option::Some(js_helpers_sub_object) => match js_helpers_index.as_string().or_else(|| js_helpers_index.as_f64().map(|x| x.to_string())) {
                        ::std::option::Option::Some(js_helpers_index_name) => match $crate::js!($($rest)*) {
                            $crate::JsMacroResult::Ok(x) => match $crate::js_sys::Reflect::set(&$root, &js_helpers_index_name.into(), &x) {
                                ::std::result::Result::Ok(true) => $crate::JsMacroResult::Ok(x),
                                _ => $crate::JsMacroResult::Err($crate::JsMacroError::Assign { object: $root.clone(), index: stringify!($field).into() }),
                            }
                            x => x,
                        }
                        ::std::option::Option::None => $crate::JsMacroResult::Err($crate::JsMacroError::Assign { object: $root.clone(), index: js_helpers_index }),
                    }
                    ::std::option::Option::None => $crate::JsMacroResult::Err($crate::JsMacroError::Assign { object: $root.clone(), index: js_helpers_index }),
                }
            }
            x => x,
        }
    };
    ($root:ident [ $($index:tt)* ] $($rest:tt)*) => {
        match $crate::js!($($index)*) {
            $crate::JsMacroResult::Ok(js_helpers_index) => match $crate::wasm_bindgen::JsCast::dyn_ref::<$crate::js_sys::Array>(&$root) {
                ::std::option::Option::Some(js_helpers_sub_object) => {
                    let js_helpers_index_f64 = js_helpers_index.as_f64().or_else(|| js_helpers_index.as_string().and_then(|x| x.parse::<f64>().ok())).unwrap_or_else(|| 0.5);
                    let js_helpers_index_u32 = js_helpers_index_f64 as u32;
                    match js_helpers_index_f64 == js_helpers_index_u32 as f64 {
                        true => {
                            let js_helpers_sub_object = js_helpers_sub_object.get(js_helpers_index_u32);
                            $crate::js!(js_helpers_sub_object $($rest)*)
                        }
                        false => $crate::JsMacroResult::Err($crate::JsMacroError::Lookup { object: $root.clone(), index: js_helpers_index }),
                    }
                }
                ::std::option::Option::None => match $crate::wasm_bindgen::JsCast::dyn_ref::<$crate::js_sys::Object>(&$root) {
                    ::std::option::Option::Some(js_helpers_sub_object) => match js_helpers_index.as_string().or_else(|| js_helpers_index.as_f64().map(|x| x.to_string())) {
                        ::std::option::Option::Some(js_helpers_index_name) => match $crate::js_sys::Reflect::get(&js_helpers_sub_object, &js_helpers_index_name.into()) {
                            ::std::result::Result::Ok(js_helpers_sub_object) => $crate::js!(js_helpers_sub_object $($rest)*),
                            ::std::result::Result::Err(_) => $crate::JsMacroResult::Err($crate::JsMacroError::Lookup { object: $root.clone(), index: js_helpers_index }),
                        }
                        ::std::option::Option::None => $crate::JsMacroResult::Err($crate::JsMacroError::Lookup { object: $root.clone(), index: js_helpers_index }),
                    }
                    ::std::option::Option::None => $crate::JsMacroResult::Err($crate::JsMacroError::Lookup { object: $root.clone(), index: js_helpers_index }),
                }
            }
            x => x,
        }
    };

    // --------------------------------------------------------------------------------------------------------------

    (( $($stuff:tt)* ) $($rest:tt)*) => {
        match $crate::js!($($stuff)*) {
            $crate::JsMacroResult::Ok(js_helpers_paren_value) => $crate::js!(js_helpers_paren_value $($rest)*),
            x => x,
        }
    };
    ($v:expr) => {
        $crate::JsMacroResult::Ok($crate::wasm_bindgen::JsValue::from($v.clone()))
    };
}
