use calamine::{Reader, Xlsx, open_workbook};
use currency::Currency;
fn main() {
    // Configuration
    let path = String::from("data/in.xlsx");
    let sheet = "Sheet1";

    let mut products = String::new();
    let mut product = String::new();

    // Open Excel File
    let mut excel: Xlsx<_> = open_workbook(path).unwrap();
    if let Some(Ok(r)) = excel.worksheet_range(sheet) {

        products.push_str("[");

        for row in r.rows() {
            product = String::from("");

            product.push_str("{");

            let money_int = Currency::from_str(&row[2].to_string()).unwrap();

            product.push_str(&format!("'sku':'{}', designation:'{}', price:{}, description:'{}'",
                                                row[0], row[1], row[2], format!("{} ({}), {}", row[1], row[0], money_int)));

            product.push_str("}");

            products.push_str(&product);
        }
        products.push_str("]");


        println!("{}", products)

    }
}
