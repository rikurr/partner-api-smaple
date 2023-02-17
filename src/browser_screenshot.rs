use headless_chrome::protocol::cdp::Page;
use headless_chrome::Browser;
use std::{error::Error, fs};
use url::Url;

pub fn get(url: &Url) -> Result<(), Box<dyn Error>> {
    // ブラウザとタブの初期化
    let browser = Browser::default()?;
    let tab = browser.new_tab()?;
    tab.set_default_timeout(std::time::Duration::from_secs(200));

    // Googleを開く
    tab.navigate_to(url.as_str())?;
    tab.wait_until_navigated()?;
    let jpeg_data =
        tab.capture_screenshot(Page::CaptureScreenshotFormatOption::Jpeg, None, None, true)?;
    fs::write("screenshot.jpg", &jpeg_data)?;

    // 検索テキストボックスへフォーカス
    tab.wait_for_element("input[name=q]")?.click()?;
    // テキストボックスへ入力
    tab.type_str("DevelopersIO")?;
    let jpeg_data =
        tab.capture_screenshot(Page::CaptureScreenshotFormatOption::Jpeg, None, None, true)?;
    fs::write("screenshot1-2.jpg", &jpeg_data)?;

    Ok(())
}
