extern crate libc;

use libc::c_char;
use std::ffi::CString;
use std::iter;
use currency::Currency;

use calamine::{Reader, Xlsx, open_workbook};

#[no_mangle]
pub extern "C" fn get() -> *mut c_char {

    let mut products = String::new();
    let mut product = String::new();

    let mut lines = 0;

    // Open Excel File
    let mut excel: Xlsx<_> = open_workbook("data/in.xlsx").unwrap();
    if let Some(Ok(r)) = excel.worksheet_range("Sheet1") {
        products.push_str("[");

        for row in r.rows().skip(1) {
            product = String::new();
            // Fields are not blank
            assert_ne!(row[0].to_string().len(), 0);
            assert_ne!(row[1].to_string().len(), 0);
            assert_ne!(row[2].to_string().len(), 0);

            product.push_str("{");

            let money_int = Currency::from_str(&row[2].to_string()).unwrap();

            product.push_str(&format!("'sku':'{}', 'designation':'{}', 'price':{}, 'description':'{}'",
                                      row[0], row[1], row[2], format!("{} ({}), ${}", row[1], row[0], money_int)));

            product.push_str("},");

            products.push_str(&product);
            lines += 1;
        }

        // Everything in excel is stored.
        assert_eq!(lines, 3);

        products.push_str("]");
    }

    let c_str = CString::new(products).unwrap();
    c_str.into_raw()
}

#[no_mangle]
pub extern "C" fn get_free(s: *mut c_char) {
    unsafe {
        if s.is_null() {
            return;
        }
        CString::from_raw(s)
    };
}