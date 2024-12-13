use js_sys::Proxy;
use wasm_bindgen::prelude::*;

struct ValueHolder {
    value: JsValue,
    ondrop: js_sys::Function,
}

impl ValueHolder {
    fn hold(&self) {
        //
    }
}

impl Drop for ValueHolder {
    fn drop(&mut self) {
        self.ondrop
            .call1(&wasm_bindgen::JsValue::UNDEFINED, &self.value)
            .unwrap();
    }
}

#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn onDrop(target: &JsValue, ondrop: &js_sys::Function) -> Proxy {
    let handler = js_sys::Object::new();
    let value_holder = ValueHolder {
        value: target.clone(),
        ondrop: ondrop.clone(),
    };
    let get_property = Closure::wrap(Box::new(
        move |target: JsValue, property: JsValue, _receiver: JsValue| {
            value_holder.hold();
            return js_sys::Reflect::get(&target, &property).unwrap();
        },
    )
        as Box<dyn FnMut(JsValue, JsValue, JsValue) -> JsValue>)
    .into_js_value();
    js_sys::Reflect::set(&handler, &JsValue::from_str("get"), &get_property).unwrap();
    return Proxy::new(target, &handler);
}
