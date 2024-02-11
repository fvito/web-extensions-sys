use js_sys::{Function, Object};
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use crate::{EventTarget};

#[wasm_bindgen]
extern "C" {
    pub type Notifications;

    #[wasm_bindgen(method)]
    pub async fn clear(
        this: &Notifications,
        notification_id: &str,
        callback: Option<&Function>,
    ) -> JsValue;

    #[wasm_bindgen(method)]
    pub async fn create(
        this: &Notifications,
        notification_id: Option<&str>,
        options: &Object,
        callback: Option<&Function>,
    ) -> JsValue;

    #[wasm_bindgen(method, js_name=getAll)]
    pub async fn get_all(
        this: &Notifications,
    ) -> JsValue;

    #[wasm_bindgen(method, js_name=getPermissionLevel)]
    pub async fn get_permission_level(
        this: &Notifications,
    ) -> JsValue;

    #[wasm_bindgen(method)]
    pub async fn update(
        this: &Notifications,
        notification_id: &str,
        options: &Object,
        callback: Option<&Function>,
    ) -> JsValue;

    #[wasm_bindgen(method, getter, js_name = onButtonClicked)]
    pub fn on_button_clicked(
        this: &Notifications
    ) -> EventTarget;

    #[wasm_bindgen(method, getter, js_name = onClicked)]
    pub fn on_clicked(
        this: &Notifications
    ) -> EventTarget;

    #[wasm_bindgen(method, getter, js_name = onClosed)]
    pub fn on_closed(
        this: &Notifications
    ) -> EventTarget;

    #[wasm_bindgen(method, getter, js_name = onPermissionLevelChanged)]
    pub fn on_permission_level_changed(
        this: &Notifications
    ) -> EventTarget;
}