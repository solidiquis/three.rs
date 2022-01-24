mod common;
use common::{logger, headless_chrome_helper};
use std::error::Error;

#[test]
fn test_gl_context() {
    logger::init_logger();

    let result = match run_test() {
        Ok(i) => i,
        Err(e) => {
            log::error!("Test failed with err: {}", e);
            false
        }
    };

    assert!(result);
}

fn run_test() -> Result<bool, Box<dyn Error>> {
    let chrome = headless_chrome_helper::HeadlessChromeHelper::new()?;
    chrome.navigate_to("http://localhost:8080")?;

    let remote_object = chrome.tab.evaluate(r#"
         wasmModule.then(wasm => wasm.tryInitGlContext("root-canvas"))
    "#, true)?;

    Ok(remote_object.description == Some("WebGLRenderingContext".to_string()))
}
