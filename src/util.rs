#![allow(clippy::missing_panics_doc)]

use seed::prelude::web_sys::HtmlInputElement;
#[allow(clippy::wildcard_imports)]
use seed::{prelude::*, *};

pub fn set_f64_on_input(name: &str, value: f64) {
    if let Ok(element) = window()
        .document()
        .expect("html document not found")
        .get_element_by_id(name)
        .unwrap_or_else(|| panic!("element {} not found", name))
        .dyn_into::<HtmlInputElement>()
    {
        element.set_value(&value.to_string());
    }
}

pub fn set_u32_on_input(name: &str, value: u32) {
    if let Ok(element) = window()
        .document()
        .expect("html document not found")
        .get_element_by_id(name)
        .unwrap_or_else(|| panic!("element {} not found", name))
        .dyn_into::<HtmlInputElement>()
    {
        element.set_value(&value.to_string());
    }
}

#[must_use]
pub fn get_f64_from_input(name: &str) -> Option<f64> {
    if let Ok(element) = window()
        .document()
        .expect("html document not found")
        .get_element_by_id(name)
        .unwrap_or_else(|| panic!("element {} not found", name))
        .dyn_into::<HtmlInputElement>()
    {
        match element.value().parse::<f64>() {
            Ok(value) => Some(value),
            Err(err) => {
                log!(format!("failed to convert {}: {}", name, err));
                None
            }
        }
    } else {
        log!(format!("failed to retrieve element {}", name));
        None
    }
}

#[must_use]
pub fn get_u32_from_input(name: &str) -> Option<u32> {
    if let Ok(element) = window()
        .document()
        .expect("html document not found")
        .get_element_by_id(name)
        .unwrap_or_else(|| panic!("element {} not found", name))
        .dyn_into::<HtmlInputElement>()
    {
        match element.value().parse::<u32>() {
            Ok(value) => Some(value),
            Err(err) => {
                log!(format!("failed to convert {}: {}", name, err));
                None
            }
        }
    } else {
        log!(format!("failed to retrieve element {}", name));
        None
    }
}

#[must_use]
pub fn find_escape_radius(c_norm: f64) -> f64 {
    // Newton iteration
    let mut radius = 2.0;

    // eprintln!("find_escape_radius({}): c_norm: {}, start: {}", c, c_norm, radius);
    for _idx in 0..20 {
        let delta_r = radius * radius - radius - c_norm;

        if (0.0..=0.01).contains(&delta_r) {
            break;
        }

        let gradient = 2.0 * radius - 1.0;
        if gradient == 0.0 {
            log!("stuck on the zero gradient");
            radius = 2.0;
            break;
        }

        radius -= delta_r / gradient;
    }

    if radius * radius - radius - c_norm >= 0.0 && radius <= 2.0 {
        radius
    } else {
        2.0
    }
}


#[cfg(test)]
mod test {
    use super::find_escape_radius;
    use crate::complex::Complex;

    #[test]
    fn test_find_escape_radius() {
        let c_norm = Complex::new(0.3, -0.5).norm();
        let radius = find_escape_radius(c);
        assert!(radius * radius - radius >= c_norm);
        assert!(radius * radius - radius - c_norm <= 0.01);

        let c_norm = Complex::new(1.0, -1.0).norm();
        let radius = find_escape_radius(c);
        assert!(radius * radius - radius >= c_norm);
        assert!(radius * radius - radius - c_norm <= 0.01);
    }
}
