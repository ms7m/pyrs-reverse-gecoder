use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

// Reverse Geocode Library
use reverse_geocoder::{Locations, ReverseGeocoder};

#[pyfunction]
fn bulk_reverse_geocode(
    items_needed: Vec<(f64, f64)>,
) -> PyResult<Vec<(String, String, String, String)>> {
    let locations = Locations::from_memory();
    let geocoder = ReverseGeocoder::new(&locations);
    let mut returned_items = Vec::new();
    for coord in items_needed {
        let returned_search_result = geocoder.search(coord).unwrap();
        let city = returned_search_result.record.admin1.clone();
        let state = returned_search_result.record.admin2.clone();
        let country = returned_search_result.record.admin3.clone();
        let formatted = format!(
            "{}, {}, {}",
            returned_search_result.record.admin1,
            returned_search_result.record.admin2,
            returned_search_result.record.admin3
        );

        let mut_items = (city, state, country, formatted);
        returned_items.push(mut_items);
    }
    Ok(returned_items)
}

#[pyfunction]
fn reverse_geocode(lat: f64, long: f64) -> PyResult<(String, String, String, String)> {
    let locations = Locations::from_memory();
    let geocoder = ReverseGeocoder::new(&locations);
    let coords = (lat, long);

    let returned_search_result = geocoder.search(coords).unwrap();
    let city = returned_search_result.record.admin1.clone();
    let state = returned_search_result.record.admin2.clone();
    let country = returned_search_result.record.admin3.clone();
    let formatted = format!(
        "{}, {}, {}",
        returned_search_result.record.admin1,
        returned_search_result.record.admin2,
        returned_search_result.record.admin3
    );

    Ok((city, state, country, formatted))
}

#[pymodule]
fn pyrs_reverse_geocoder(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(reverse_geocode, m)?)?;
    m.add_function(wrap_pyfunction!(bulk_reverse_geocode, m)?)?;
    Ok(())
}
