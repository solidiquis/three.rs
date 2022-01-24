mod common;
use common::{logger, headless_chrome_helper, webpack_dev_server};
use std::error::Error;

#[test]
fn test_gl_context() {
    let webpack_dev_server = webpack_dev_server::WebpackDevServer::default();
    logger::init_logger();

    let result = match run_test(&webpack_dev_server) {
        Ok(i) => i,
        Err(e) => {
            log::error!("Test failed with err: {}", e);
            false
        }
    };

    webpack_dev_server.kill();
    assert!(result);
}

fn run_test(webpack_dev_server: &webpack_dev_server::WebpackDevServer) -> Result<bool, Box<dyn Error>> {
    webpack_dev_server.init()?;

    let chrome = headless_chrome_helper::HeadlessChromeHelper::new()?;
    let tab = chrome.nav_to("http://localhost:8080")?;

    let remote_object = tab.evaluate(r#"
         wasmModule.then(wasm => wasm.tryInitGlContext("root-canvas"))
    "#, true)?;

    Ok(remote_object.description == Some("WebGLRenderingContext".to_string()))
}
