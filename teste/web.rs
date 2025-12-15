#![allow(dead_code)]

use counter::*;
use leptos::{mount::mount_to, prelude::*, task::tick};
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn clear() {
    let document = document();
    let test_wrapper = document.create_element("section").unwrap();
    let _ = document.body().unwrap().append_child(&test_wrapper);

    let _dispose = mount_to(
        test_wrapper.clone().unchecked_into(),
        || view! { <SimpleCounter initial_value=10 step=1/> },
    );

    let div = test_wrapper.query_selector("div").unwrap().unwrap();
    let clear = test_wrapper
        .query_selector("button")
        .unwrap()
        .unwrap()
        .unchecked_into::<web_sys::HtmlElement>();

    clear.click();

    tick().await;

    assert_eq!(div.outer_html(), {
        let (value, _set_value) = signal(0);

        view! {
            <div>
                <button>"Clear"</button>
                <button>"-1"</button>
                <span>"Value: " {value} "!"</span>
                <button>"+1"</button>
            </div>
        }
        .into_view()
        .build()
        .outer_html()
    });

    assert_eq!(test_wrapper.inner_html(), {
        let comparison_wrapper = document.create_element("section").unwrap();
        let _dispose = mount_to(
            comparison_wrapper.clone().unchecked_into(),
            || view! { <SimpleCounter initial_value=0 step=1/>},
        );
        comparison_wrapper.inner_html()
    });
}

#[wasm_bindgen_test]
async fn inc() {
    let document = document();
    let test_wrapper = document.create_element("section").unwrap();
    let _ = document.body().unwrap().append_child(&test_wrapper);

    let _dispose = mount_to(
        test_wrapper.clone().unchecked_into(),
        || view! { <SimpleCounter initial_value=0 step=1/> },
    );

    // You can do testing with vanilla DOM operations
    let div = test_wrapper.query_selector("div").unwrap().unwrap();
    let clear = div
        .first_child()
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()
        .unwrap();
    let dec = clear
        .next_sibling()
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()
        .unwrap();
    let text = dec
        .next_sibling()
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()
        .unwrap();
    let inc = text
        .next_sibling()
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()
        .unwrap();

    inc.click();
    inc.click();

    tick().await;

    assert_eq!(text.text_content(), Some("Value: 2!".to_string()));

    dec.click();
    dec.click();
    dec.click();
    dec.click();

    tick().await;

    assert_eq!(text.text_content(), Some("Value: -2!".to_string()));

    clear.click();

    tick().await;

    assert_eq!(text.text_content(), Some("Value: 0!".to_string()));

    assert_eq!(
        div.outer_html(),
        {
            let (value, _) = signal(0);
            view! {
                <div>
                    <button>"Clear"</button>
                    <button>"-1"</button>
                    <span>"Value: " {value} "!"</span>
                    <button>"+1"</button>
                </div>
            }
        }
        .into_view()
        .build()
        .outer_html()
    );

    inc.click();

    tick().await;

    assert_eq!(
        div.outer_html(),
        {
            // because we've clicked, it's as if the signal is starting at 1
            let (value, _) = signal(1);
            view! {
                <div>
                    <button>"Clear"</button>
                    <button>"-1"</button>
                    <span>"Value: " {value} "!"</span>
                    <button>"+1"</button>
                </div>
            }
        }
        .into_view()
        .build()
        .outer_html()
    );
}
