fn main() {
    enum Spreadsheetcell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        Spreadsheetcell::Int(3),
        Spreadsheetcell::Text(String::from("blue")),
        Spreadsheetcell::Float(10.12),
    ];
}
