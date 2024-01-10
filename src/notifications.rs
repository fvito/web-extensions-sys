use js_sys::{Function, Object};
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use crate::{ContextMenus, EventTarget};

#[wasm_bindgen]
extern "C" {
    pub type Notifications;

    #[wasm_bindgen(method)]
    pub fn clear(
        this: &Notifications,
        notification_id: &JsValue,
        callback: Option<&Function>,
    );

    #[wasm_bindgen(method)]
    pub fn create(
        this: &Notifications,
        notification_id: &JsValue,
        options: Option<&Object>,
        callback: Option<&Function>,
    ) -> JsValue;

    #[wasm_bindgen(method, js_name=getAll)]
    pub fn get_all(
        this: &Notifications,
        callback: Option<&Function>,
    );

    #[wasm_bindgen(method, js_name=getPermissionLevel)]
    pub fn get_permission_level(
        this: &Notifications,
        callback: Option<&Function>,
    );

    #[wasm_bindgen(method)]
    pub fn update(
        this: &Notifications,
        notification_id: &JsValue,
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