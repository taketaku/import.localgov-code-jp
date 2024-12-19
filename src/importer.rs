use calamine::{DataType, Reader, Xlsx};
use rusqlite::{params, Connection, Result};
use std::{fs::File, io::BufReader};

pub fn import_sheet_to_table(
    conn: &Connection,
    workbook: &mut Xlsx<BufReader<File>>,
    sheet_name: &str,
    table_name: &str,
) -> Result<()> {
    let range = workbook
        .worksheet_range(sheet_name)
        .expect("Cannot find the worksheet");

    let tx = conn.unchecked_transaction()?;
    {
        let sql = format!(
            "INSERT OR REPLACE INTO {} 
            (code, prefecture_name, prefecture_name_kana, name, name_kana)
            VALUES (?1, ?2, ?3, ?4, ?5)",
            table_name
        );
        let mut stmt = tx.prepare(&sql)?;

        for (i, row) in range.rows().enumerate() {
            if i == 0 {
                continue;
            }

            let code = row.get(0).and_then(|c| c.get_string()).unwrap_or("");
            let pref_name = row.get(1).and_then(|c| c.get_string()).unwrap_or("");
            let pref_name_kana = row.get(4).and_then(|c| c.get_string()).unwrap_or("");
            let city_name = row.get(2).and_then(|c| c.get_string()).unwrap_or("");
            let city_name_kana = row.get(3).and_then(|c| c.get_string()).unwrap_or("");

            stmt.execute(params![
                code,
                pref_name,
                pref_name_kana,
                city_name,
                city_name_kana
            ])?;
        }
    }
    tx.commit()?;
    Ok(())
}
