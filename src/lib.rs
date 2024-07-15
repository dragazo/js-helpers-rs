pub use wasm_bindgen;
pub use web_sys::{self, js_sys};

pub enum JsMacroError {

}

#[macro_export]
macro_rules! js {
    (null) => {
        $crate::wasm_bindgen::JsValue::null()
    };
    (undefined) => {
        $crate::wasm_bindgen::JsValue::undefined()
    };
    ($v:ident) => {
        $crate::wasm_bindgen::JsValue::from($v.clone())
    };

    // --------------------------------------------------------------------------------------------------------------

    ([ $($t:tt)* ]) => {{
        let mut js_helpers_array_target = $crate::js_sys::Array::new();
        js!(@fill_array js_helpers_array_target $($t)*);
        $crate::wasm_bindgen::JsValue::from(js_helpers_array_target)
    }};
    (@fill_array $target:ident) => {};
    (@fill_array $target:ident $v:ident $(, $($rest:tt)*)?) => {{
        $target.push(&js!($v));
        js!(@fill_array $target $($($rest)*)?);
    }};
    (@fill_array $target:ident [ $($t:tt)* ] $(, $($rest:tt)*)?) => {{
        $target.push(&js!([ $($t)* ]));
        js!(@fill_array $target $($($rest)*)?);
    }};
    (@fill_array $target:ident { $($t:tt)* } $(, $($rest:tt)*)?) => {{
        $target.push(&js!({ $($t)* }));
        js!(@fill_array $target $($($rest)*)?);
    }};
    (@fill_array $target:ident $v:expr $(, $($rest:tt)*)?) => {{
        $target.push(&js!($v));
        js!(@fill_array $target $($($rest)*)?);
    }};

    // --------------------------------------------------------------------------------------------------------------

    ({ $($t:tt)* }) => {{
        let mut js_helpers_object_target = $crate::js_sys::Map::new();
        js!(@fill_object js_helpers_object_target $($t)*);
        $crate::wasm_bindgen::JsValue::from(js_helpers_object_target)
    }};
    (@fill_object $target:ident) => {};
    (@fill_object $target:ident $key:ident : $value:ident $(, $($rest:tt)*)?) => {{
        $target.set(&$crate::wasm_bindgen::JsValue::from(stringify!($key)), &js!($value));
        js!(@fill_object $target $($($rest)*)?);
    }};
    (@fill_object $target:ident $key:ident : [ $($t:tt)* ] $(, $($rest:tt)*)?) => {{
        $target.set(&$crate::wasm_bindgen::JsValue::from(stringify!($key)), &js!([ $($t)* ]));
        js!(@fill_object $target $($($rest)*)?);
    }};
    (@fill_object $target:ident $key:ident : { $($t:tt)* } $(, $($rest:tt)*)?) => {{
        $target.set(&$crate::wasm_bindgen::JsValue::from(stringify!($key)), &js!({ $($t)* }));
        js!(@fill_object $target $($($rest)*)?);
    }};
    (@fill_object $target:ident $key:ident : $value:expr $(, $($rest:tt)*)?) => {{
        $target.set(&$crate::wasm_bindgen::JsValue::from(stringify!($key)), &js!($value));
        js!(@fill_object $target $($($rest)*)?);
    }};

    // --------------------------------------------------------------------------------------------------------------

    ($v:expr) => {
        $crate::wasm_bindgen::JsValue::from($v)
    };

    // --------------------------------------------------------------------------------------------------------------





    // ($root:ident) => {
    //     $root.clone()
    // };
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

