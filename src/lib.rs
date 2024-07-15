pub use wasm_bindgen;
pub use web_sys::{self, js_sys};

#[derive(Debug)]
pub enum JsMacroError {
    Lookup { name: &'static str },
}
pub type JsMacroResult = Result<wasm_bindgen::JsValue, JsMacroError>;

#[macro_export]
macro_rules! js {
    (null) => {
        $crate::JsMacroResult::Ok($crate::wasm_bindgen::JsValue::null())
    };
    (undefined) => {
        $crate::JsMacroResult::Ok($crate::wasm_bindgen::JsValue::undefined())
    };
    ($v:ident) => {
        $crate::JsMacroResult::Ok($crate::wasm_bindgen::JsValue::from($v.clone()))
    };

    // --------------------------------------------------------------------------------------------------------------

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

    ($root:ident . $field:ident $($rest:tt)*) => {{
        match $crate::js_sys::Reflect::get(&$root, &$crate::wasm_bindgen::JsValue::from(stringify!($field))) {
            ::std::result::Result::Ok(js_helpers_sub_object) => js!(js_helpers_sub_object $($rest)*),
            ::std::result::Result::Err(e) => $crate::JsMacroResult::Err($crate::JsMacroError::Lookup { name: stringify!($field) }),
        }
    }};

    // --------------------------------------------------------------------------------------------------------------

    ($v:expr) => {
        $crate::JsMacroResult::Ok($crate::wasm_bindgen::JsValue::from($v))
    };



    // ($root:ident . $f:ident ( $($args:expr),*$(,)? )) => {
    //     Reflect::apply(&js!($root.$f).dyn_ref().unwrap(), &$root, &(vec![$($args),*] as Vec<JsValue>).into_iter().collect())
    // };
    // ($root:ident . $path:ident $($rest:tt)*) => {{
    //     let zzz = Reflect::get(&$root, &stringify!($path).into()).unwrap_or_else(|_| JsValue::undefined());
    //     js!(zzz $($rest)*)
    // }};
    // ($root:ident [ $idx:expr ] $($rest:tt)*) => {{
    //     let zzz = $root.dyn_ref::<Array>().map(|arr| arr.get($idx)).unwrap_or_else(|| JsValue::undefined());
    //     js!(zzz $($rest)*)
    // }};
}

