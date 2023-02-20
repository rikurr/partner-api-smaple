use headless_chrome::types::Bounds;
use headless_chrome::Browser;
use headless_chrome::{protocol::cdp::Page, LaunchOptionsBuilder};
use std::{error::Error, fs};
use url::Url;

pub fn get(url: &Url) -> Result<(), Box<dyn Error>> {
    // ブラウザの初期化
    let browser = Browser::new(
        LaunchOptionsBuilder::default()
            .headless(true)
            .build()
            .unwrap(),
    )
    .unwrap();

    // タブの作成
    let tab = browser.new_tab()?;

    // ページ遷移
    tab.navigate_to(url.as_str())?;
    tab.wait_until_navigated()?;

    // ページの高さを取得
    let scroll_height = tab
        .evaluate(
            "Math.max(document.documentElement.scrollHeight, document.body.scrollHeight)",
            true,
        )
        .unwrap()
        .value
        .unwrap()
        .as_i64()
        .unwrap() as u32;

    // タブの高さを取得
    let viewport_height = tab
        .evaluate("window.innerHeight", true)
        .unwrap()
        .value
        .unwrap()
        .as_i64()
        .unwrap() as u32;

    // ボティ要素のviewportを取得
    let body = tab.wait_for_element("body")?;
    let viewport = body.get_box_model()?.content_viewport();
    tab.set_bounds(Bounds::Normal {
        left: Some(0),
        top: Some(0),
        width: Some(1000.0),
        height: Some(viewport.height + viewport_height as f64),
    })?;

    // println!("{}", scroll_height);
    // println!("{}", viewport_height);
    // println!("{:?}", &viewport);

    // ページを一番下までスクロールする
    let mut scroll = 0;
    while scroll + viewport_height < scroll_height {
        let expression = format!(
            "
        () => {{
            window.scrollTo({{
                top: {},
                behavior: 'smooth
            }});
        }})
        ",
            viewport_height
        );
        tab.evaluate(&expression, true).unwrap();
        scroll += viewport_height;
    }

    // スクリーンショットを撮る
    let jpeg_data =
        tab.capture_screenshot(Page::CaptureScreenshotFormatOption::Jpeg, None, None, true)?;

    // スクリーンショットの書き出し
    let domain = url.domain().unwrap();
    let path = format!("screenshot/{}.jpg", domain);
    fs::write(path, jpeg_data)?;

    // タブを閉じる
    tab.close(true).unwrap();
    Ok(())
}
