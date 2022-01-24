use headless_chrome::Browser;
use headless_chrome::browser::tab::Tab;
use headless_chrome::util::Wait;
use serde_json::value::Value;
use std::sync::Arc;
use std::error::Error;

pub struct HeadlessChromeHelper {
    pub browser: Browser,
    pub tab: Arc<Tab>
}

impl HeadlessChromeHelper {
    pub fn new() -> Result<Self, String> {
        let browser = Browser::default()
            .or_else(|err| Err(format!["Failed to launch headless chrome with err: {}", err]))?;

        let tab = browser.wait_for_initial_tab()
            .or_else(|err| Err(format!["HeadlessChromeHelper#nav_to failed to wait for initial tab with err: {}", err]))?;

        Ok(Self { browser, tab })
    }

    pub fn navigate_to<S: AsRef<str>>(&self, path: S) -> Result<(), String> {
        self.tab.navigate_to(path.as_ref())
            .or_else(|err| Err(format!["HeadlessChromeHelper#nav_to failed navigate to {} with err: {}", &path.as_ref(), err]))?;

        let tab_ref = &self.tab;

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

        Ok(())
    }
}

