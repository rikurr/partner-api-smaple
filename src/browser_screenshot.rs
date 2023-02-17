use headless_chrome::Browser;
use headless_chrome::{protocol::cdp::Page, LaunchOptionsBuilder};
use std::{error::Error, fs};
use url::Url;

pub fn get(url: &Url) -> Result<(), Box<dyn Error>> {
    // ブラウザとタブの初期化
    let options = LaunchOptionsBuilder::default()
        // Make the window bigger
        .window_size(Some((1920, 5000)))
        .build()?;
    let browser = Browser::new(options)?;
    let tab = browser.new_tab()?;
    let domain = url.domain().unwrap();
    let path = format!("screenshot/{}.jpg", domain);
    tab.set_default_timeout(std::time::Duration::from_secs(200));
    tab.navigate_to(url.as_str())?;
    tab.wait_until_navigated()?;

    let expression = r#"
    () => {
        window.scrollTo(0, document.body.scrollHeight);
        }
        "#;
    // ページを一番下までスクロールする
    tab.evaluate(expression, false)?;

    let body = tab.wait_for_element("html")?;
    let viewport = body.get_box_model()?.margin_viewport();

    println!("{:?}", viewport);

    let jpeg_data = tab.capture_screenshot(
        Page::CaptureScreenshotFormatOption::Jpeg,
        None,
        Some(viewport),
        true,
    )?;
    fs::write(path, &jpeg_data)?;

    Ok(())
}
