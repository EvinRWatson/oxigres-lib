use std::ffi::CString;
use std::ffi::CStr;
use postgres::Row;
use std::os::raw::c_char;

#[path = "./db_deserializer.rs"]
mod db_deserializer;
#[path = "./db_manager.rs"]
mod db_manager;

#[no_mangle]
pub extern "C" fn query_database(query: *const c_char) -> *mut c_char {

    let mut pg_client = db_manager::postgres_connect();

    let query_string = unsafe { CStr::from_ptr(query) };

    let postgres_response: Vec<Row> = match pg_client.query(query_string.to_str().unwrap(), &[]) {
        Ok(value) => value,
        Err(error) => return CString::new(db_deserializer::error_json(&error.to_string())).unwrap().into_raw()
    };

    let json_response: String = match db_deserializer::convert_response_to_json_string(&postgres_response) {
        Ok(value) => value,
        Err(error) => return CString::new(db_deserializer::error_json(&error)).unwrap().into_raw()
    };

    match CString::new(json_response) {
        Ok(value) => return value.into_raw(),
        Err(_) => return CString::new("CString Conversion Error").unwrap().into_raw(),
    };
}