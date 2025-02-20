use serde::{de::DeserializeOwned, Serialize};
use url::Url;

/// Convert a device file path to an URL that can be loaded by the webview.
/// Note that `asset:` and `https://asset.localhost` must be added to [`tauri.security.csp`](https://tauri.app/v1/api/config/#securityconfig.csp) in `tauri.conf.json`.
/// Example CSP value: `"csp": "default-src 'self'; img-src 'self' asset: https://asset.localhost"` to use the asset protocol on image sources.
///
/// Additionally, `asset` must be added to [`tauri.allowlist.protocol`](https://tauri.app/v1/api/config/#allowlistconfig.protocol)
/// in `tauri.conf.json` and its access scope must be defined on the `assetScope` array on the same `protocol` object.
///
/// @param  filePath The file path.
/// @param  protocol The protocol to use. Defaults to `asset`. You only need to set this when using a custom protocol.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_api::path::{app_data_dir, join};
/// use tauri_api::tauri::convert_file_src;
///
/// const app_data_dir_path = app_data_dir().await;
/// const file_path = join(app_data_dir_path, "assets/video.mp4").await;
/// const asset_url = convert_file_src(file_path);
///
/// let window = web_sys::window().expect("no global `window` exists");
/// let document = window.document().expect("should have a document on window");
///
/// // Manufacture the element we're gonna append
/// let video = document.get_element_by_id("my-video")?;
/// let source = document.create_element("source")?;
///
/// source.set_attribute("type", "video/mp4")?;
/// source.set_attribute("src", asset_url.as_str())?;
///
/// video.append_child(&val)?;
/// ```
///
/// @return the URL that can be used as source on the webview.
#[inline(always)]
pub async fn convert_file_src(file_path: &str, protocol: Option<&str>) -> Url {
    Url::parse(
        &inner::convertFileSrc(file_path, protocol)
            .await
            .as_string()
            .unwrap(),
    )
    .unwrap()
}

/// Sends a message to the backend.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_api::tauri::invoke;
///
/// struct User<'a> {
///     user: &'a str,
///     password: &'a str
/// }
///
/// invoke("login", &User { user: "tauri", password: "poiwe3h4r5ip3yrhtew9ty" }).await;
/// ```
///
/// @param cmd The command name.
/// @param args The optional arguments to pass to the command.
/// @return A promise resolving or rejecting to the backend response.
#[inline(always)]
pub async fn invoke<A: Serialize, R: DeserializeOwned>(cmd: &str, args: &A) -> crate::Result<R> {
    let res = inner::invoke(cmd, serde_wasm_bindgen::to_value(args).unwrap()).await;

    let raw = res.map_err(crate::Error::Other)?;
    serde_wasm_bindgen::from_value(raw).map_err(Into::into)
}

/// Transforms a callback function to a string identifier that can be passed to the backend.
/// The backend uses the identifier to `eval()` the callback.
///
/// @return A unique identifier associated with the callback function.
#[inline(always)]
pub async fn transform_callback<T: DeserializeOwned>(callback: &dyn Fn(T), once: bool) -> f64 {
    inner::transformCallback(
        &|raw| callback(serde_wasm_bindgen::from_value(raw).unwrap()),
        once,
    )
    .await
    .as_f64()
    .unwrap()
}

mod inner {
    use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

    #[wasm_bindgen(module = "/dist/tauri.js")]
    extern "C" {
        pub async fn convertFileSrc(filePath: &str, protocol: Option<&str>) -> JsValue;
        #[wasm_bindgen(catch)]
        pub async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
        pub async fn transformCallback(callback: &dyn Fn(JsValue), once: bool) -> JsValue;
    }
}
