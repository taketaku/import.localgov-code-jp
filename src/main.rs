use calamine::{open_workbook, Xlsx};
use rusqlite::{Connection, Result};

mod importer;

const MAIN_TABLE: &str = "jichitai_codes";
const DESIGNATED_TABLE: &str = "jichitai_codes_designated";
const DOWNLOADED_FILE: &str = "downloaded/jichitai_code_list.xlsx";
const SHEET_1: &str = "R6.1.1現在の団体";
const SHEET_2: &str = "R6.1.1政令指定都市";

// 団体コード、都道府県名、都道府県名(カナ)、市区町村名、市区町村名(カナ)
fn create_table(conn: &Connection, table_name: &str) -> Result<()> {
    let sql = format!(
        "CREATE TABLE IF NOT EXISTS {} (
            code TEXT PRIMARY KEY,
            prefecture_name TEXT,
            prefecture_name_kana TEXT,
            name TEXT,
            name_kana TEXT
        )",
        table_name
    );
    conn.execute(&sql, [])?;
    Ok(())
}

fn main() -> Result<()> {
    let conn = Connection::open("localgov-code.db")?;

    conn.execute(&format!("DROP TABLE IF EXISTS {}", MAIN_TABLE), [])?;
    conn.execute(&format!("DROP TABLE IF EXISTS {}", DESIGNATED_TABLE), [])?;
    create_table(&conn, MAIN_TABLE)?;
    create_table(&conn, DESIGNATED_TABLE)?;

    let mut workbook: Xlsx<_> = open_workbook(DOWNLOADED_FILE).expect("Cannot open Excel file");

    importer::import_sheet_to_table(&conn, &mut workbook, SHEET_1, MAIN_TABLE)?;
    importer::import_sheet_to_table(&conn, &mut workbook, SHEET_2, DESIGNATED_TABLE)?;

    println!("Import completed successfully.");
    Ok(())
}
