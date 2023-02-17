use headless_chrome::protocol::cdp::Page;
use headless_chrome::Browser;
use std::{error::Error, fs};
use url::Url;

pub fn get(url: &Url) -> Result<(), Box<dyn Error>> {
    // ブラウザとタブの初期化
    let browser = Browser::default()?;
    let tab = browser.new_tab()?;
    let domain = url.domain().unwrap();
    let path = format!("screenshot/{}.jpg", domain);
    tab.set_default_timeout(std::time::Duration::from_secs(200));

    let viewport = tab
        .navigate_to(url.as_str())?
        .wait_for_element("body")?
        .get_box_model()?
        .margin_viewport();
    tab.wait_until_navigated()?;
    let jpeg_data = tab.capture_screenshot(
        Page::CaptureScreenshotFormatOption::Jpeg,
        None,
        Some(viewport),
        true,
    )?;
    fs::write(path, &jpeg_data)?;

    Ok(())
}
