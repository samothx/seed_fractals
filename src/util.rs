use seed::{prelude::*, *};
use seed::window;
use seed::prelude::{web_sys::HtmlInputElement};
use seed::log;

pub fn get_f64_from_input(name: &str) -> Option<f64> {
    if let Ok(element) = window().document().expect("html document not found")
        .get_element_by_id(name).expect(format!("element {} not found", name).as_str()).dyn_into::<HtmlInputElement>() {
        match element.value().parse::<f64>() {
            Ok(value) => Some(value),
            Err(err) => {
                log!(format!("failed to convert {}: {}",name, err));
                None
            }
        }
    } else {
        log!(format!("failed to retrieve element {}",name));
        None
    }
}

pub fn get_u32_from_input(name: &str) -> Option<u32> {
    if let Ok(element) = window().document().expect("html document not found")
        .get_element_by_id(name).expect(format!("element {} not found", name).as_str()).dyn_into::<HtmlInputElement>() {
        match element.value().parse::<u32>() {
            Ok(value) => Some(value),
            Err(err) => {
                log!(format!("failed to convert {}: {}",name, err));
                None
            }
        }
    } else {
        log!(format!("failed to retrieve element {}",name));
        None
    }
}