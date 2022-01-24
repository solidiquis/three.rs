use headless_chrome::Browser;
use headless_chrome::browser::tab::Tab;
use headless_chrome::util::Wait;
use serde_json::value::Value;
use std::sync::Arc;
use std::error::Error;

/// Simple wrapper around headless chrome that takes care of the tedious task
/// of checking to see when WASM module is finished downloading.
pub struct HeadlessChromeHelper(Browser);

impl HeadlessChromeHelper {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let browser = Browser::default()?;
        Ok(Self(browser))
    }

    /// Navigates to path and waits until wasmModule is successfully instantiated.
    /// If wasmModule is set, a timeout error will be raised via Wait's default settings.
    pub fn nav_to<S: AsRef<str>>(&self, path: S) -> Result<Arc<Tab>, String>  {
        let tab = self.0.wait_for_initial_tab()
            .or_else(|err| Err(format!["HeadlessChromeHelper#nav_to failed to wait for initial tab with err: {}", err]))?;

        tab.navigate_to(path.as_ref())
            .or_else(|err| Err(format!["HeadlessChromeHelper#nav_to failed navigate to {} with err: {}", &path.as_ref(), err]))?;

        let tab_ref = &tab;

        match Wait::default().until(|| {
            match tab_ref.evaluate("window.wasmModule != undefined", false) {
                Ok(obj) => match obj.value {
                    Some(b) if b == Value::Bool(true) => Some(()),
                    _ => None
                },
                _ => None
            }
        }) {
            Err(_) => return Err("WASM module took too long to load.".to_string()),
            _ => ()
        }

        Ok(tab)
    }
}

