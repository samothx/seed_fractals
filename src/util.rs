#[allow(clippy::wildcard_imports)]
use seed::{prelude::*, *};
use seed::prelude::web_sys::HtmlInputElement;

pub fn set_f64_on_input(name: &str,value: f64) {
    if let Ok(element) = window().document().expect("html document not found")
        .get_element_by_id(name).expect(format!("element {} not found", name).as_str()).dyn_into::<HtmlInputElement>() {
        element.set_value(&value.to_string())
    }
}

pub fn set_u32_on_input(name: &str,value: u32) {
    if let Ok(element) = window().document().expect("html document not found")
        .get_element_by_id(name).expect(format!("element {} not found", name).as_str()).dyn_into::<HtmlInputElement>() {
        element.set_value(&value.to_string())
    }
}

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
