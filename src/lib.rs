pub use wasm_bindgen;
pub use web_sys::{self, js_sys};

#[derive(Debug)]
pub enum JsMacroError {
    IndexLookup { object: wasm_bindgen::JsValue, index: wasm_bindgen::JsValue },
    DotLookup { object: wasm_bindgen::JsValue, name: &'static str },
}
pub type JsMacroResult = Result<wasm_bindgen::JsValue, JsMacroError>;

#[macro_export]
macro_rules! js {
    ([ $($t:tt)* ]) => {{
        let mut js_helpers_array_target = $crate::js_sys::Array::new();
        js!(@fill_array js_helpers_array_target $($t)*)
    }};
    (@fill_array $target:ident) => {
        $crate::JsMacroResult::Ok($crate::wasm_bindgen::JsValue::from($target))
    };
    (@fill_array $target:ident $v:ident $(, $($rest:tt)*)?) => {
        match js!($v) {
            $crate::JsMacroResult::Ok(v) => {
                $target.push(&v);
                js!(@fill_array $target $($($rest)*)?)
            }
            x => x,
        }
    };
    (@fill_array $target:ident [ $($t:tt)* ] $(, $($rest:tt)*)?) => {
        match js!([ $($t)* ]) {
            $crate::JsMacroResult::Ok(v) => {
                $target.push(&v);
                js!(@fill_array $target $($($rest)*)?)
            }
            x => x,
        }
    };
    (@fill_array $target:ident { $($t:tt)* } $(, $($rest:tt)*)?) => {
        match js!({ $($t)* }) {
            $crate::JsMacroResult::Ok(v) => {
                $target.push(&v);
                js!(@fill_array $target $($($rest)*)?)
            }
            x => x,
        }
    };
    (@fill_array $target:ident $v:expr $(, $($rest:tt)*)?) => {
        match js!($v) {
            $crate::JsMacroResult::Ok(v) => {
                $target.push(&v);
                js!(@fill_array $target $($($rest)*)?)
            }
            x => x,
        }
    };

    // --------------------------------------------------------------------------------------------------------------

    ({ $($t:tt)* }) => {{
        let mut js_helpers_object_target = $crate::wasm_bindgen::JsValue::from($crate::js_sys::Object::new());
        js!(@fill_object js_helpers_object_target $($t)*)
    }};
    (@fill_object $target:ident) => {
        $crate::JsMacroResult::Ok($target)
    };
    (@fill_object $target:ident $key:ident : $value:ident $(, $($rest:tt)*)?) => {
        match js!($value) {
            $crate::JsMacroResult::Ok(v) => {
                $crate::js_sys::Reflect::set(&$target, &$crate::wasm_bindgen::JsValue::from(stringify!($key)), &v).unwrap();
                js!(@fill_object $target $($($rest)*)?)
            }
            x => x,
        }
    };
    (@fill_object $target:ident $key:ident : [ $($t:tt)* ] $(, $($rest:tt)*)?) => {
        match js!([ $($t)* ]) {
            $crate::JsMacroResult::Ok(v) => {
                $crate::js_sys::Reflect::set(&$target, &$crate::wasm_bindgen::JsValue::from(stringify!($key)), &v).unwrap();
                js!(@fill_object $target $($($rest)*)?)
            }
            x => x,
        }
    };
    (@fill_object $target:ident $key:ident : { $($t:tt)* } $(, $($rest:tt)*)?) => {
        match js!({ $($t)* }) {
            $crate::JsMacroResult::Ok(v) => {
                $crate::js_sys::Reflect::set(&$target, &$crate::wasm_bindgen::JsValue::from(stringify!($key)), &v).unwrap();
                js!(@fill_object $target $($($rest)*)?)
            }
            x => x,
        }
    };
    (@fill_object $target:ident $key:ident : $value:expr $(, $($rest:tt)*)?) => {
        match js!($value) {
            $crate::JsMacroResult::Ok(v) => {
                $crate::js_sys::Reflect::set(&$target, &$crate::wasm_bindgen::JsValue::from(stringify!($key)), &v).unwrap();
                js!(@fill_object $target $($($rest)*)?)
            }
            x => x,
        }
    };

    // --------------------------------------------------------------------------------------------------------------

    ($root:ident . $field:ident $($rest:tt)*) => {
        match $crate::js_sys::Reflect::get(&$root, &$crate::wasm_bindgen::JsValue::from(stringify!($field))) {
            ::std::result::Result::Ok(js_helpers_sub_object) => js!(js_helpers_sub_object $($rest)*),
            ::std::result::Result::Err(_) => $crate::JsMacroResult::Err($crate::JsMacroError::DotLookup { object: $root.clone(), name: stringify!($field) }),
        }
    };
    ($root:ident ?. $field:ident $($rest:tt)*) => {{
        let js_helpers_sub_object = $crate::js_sys::Reflect::get(&$root, &$crate::wasm_bindgen::JsValue::from(stringify!($field))).unwrap_or_else(|_| $crate::wasm_bindgen::JsValue::undefined());
        js!(js_helpers_sub_object $($rest)*)
    }};

    // --------------------------------------------------------------------------------------------------------------

    ($root:ident [ $($index:tt)* ] $($rest:tt)*) => {{
        match js!($($index)*) {
            $crate::JsMacroResult::Ok(js_helpers_index) => match $crate::wasm_bindgen::JsCast::dyn_ref::<$crate::js_sys::Array>(&$root) {
                ::std::option::Option::Some(js_helpers_sub_object) => {
                    let js_helpers_index_f64 = js_helpers_index.as_f64().or_else(|| js_helpers_index.as_string().and_then(|x| x.parse::<f64>().ok())).unwrap_or_else(|| 0.5);
                    let js_helpers_index_u32 = js_helpers_index_f64 as u32;
                    match js_helpers_index_f64 == js_helpers_index_u32 as f64 {
                        true => {
                            let js_helpers_sub_object = js_helpers_sub_object.get(js_helpers_index_u32);
                            js!(js_helpers_sub_object $($rest)*)
                        }
                        false => $crate::JsMacroResult::Err($crate::JsMacroError::IndexLookup { object: $root.clone(), index: js_helpers_index }),
                    }
                }
                ::std::option::Option::None => match $crate::wasm_bindgen::JsCast::dyn_ref::<$crate::js_sys::Object>(&$root) {
                    ::std::option::Option::Some(js_helpers_sub_object) => match js_helpers_index.as_string().or_else(|| js_helpers_index.as_f64().map(|x| x.to_string())) {
                        ::std::option::Option::Some(js_helpers_index_name) => match $crate::js_sys::Reflect::get(&js_helpers_sub_object, &js_helpers_index_name.into()) {
                            ::std::result::Result::Ok(js_helpers_sub_object) => js!(js_helpers_sub_object $($rest)*),
                            ::std::result::Result::Err(_) => $crate::JsMacroResult::Err($crate::JsMacroError::IndexLookup { object: $root.clone(), index: js_helpers_index }),
                        }
                        ::std::option::Option::None => $crate::JsMacroResult::Err($crate::JsMacroError::IndexLookup { object: $root.clone(), index: js_helpers_index }),
                    }
                    ::std::option::Option::None => $crate::JsMacroResult::Err($crate::JsMacroError::IndexLookup { object: $root.clone(), index: js_helpers_index }),
                }
            }
            x => x,
        }
    }};

    // --------------------------------------------------------------------------------------------------------------

    (null) => {
        $crate::JsMacroResult::Ok($crate::wasm_bindgen::JsValue::null())
    };
    (undefined) => {
        $crate::JsMacroResult::Ok($crate::wasm_bindgen::JsValue::undefined())
    };
    ($v:ident) => {
        $crate::JsMacroResult::Ok($crate::wasm_bindgen::JsValue::from($v.clone()))
    };
    ($v:expr) => {
        $crate::JsMacroResult::Ok($crate::wasm_bindgen::JsValue::from($v))
    };



    // ($root:ident . $f:ident ( $($args:expr),*$(,)? )) => {
    //     Reflect::apply(&js!($root.$f).dyn_ref().unwrap(), &$root, &(vec![$($args),*] as Vec<JsValue>).into_iter().collect())
    // };
}
