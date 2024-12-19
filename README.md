# 都道府県コード及び市区町村コード to SQLite

[全国地方公共団体コード](https://www.soumu.go.jp/denshijiti/code.html)で取得した"都道府県コード及び市区町村コード"
(.xlsx)を SQLite データベースに置き換える

構成要素

- 団体コード(自治体コード)
- 都道府県名
- 都道府県名(カナ)
- 市区町村名
- 市区町村名(カナ)

## setup

- [Install Rust](https://www.rust-lang.org/tools/install)
- [setup editor suppoort](https://www.rust-lang.org/tools)

## how to use

1. [全国地方公共団体コード](https://www.soumu.go.jp/denshijiti/code.html)から最新の「都道府県コード及び市区町村コード」を取得
2. `downloaded/jichitai_code_list.xlsx`を置き換える
3. 必要に応じて変数を置き換える。`src/main.rs`

   ```.rs
   const MAIN_TABLE: &str = "jichitai_codes";
   const DESIGNATED_TABLE: &str = "jichitai_codes_designated";
   const DOWNLOADED_FILE: &str = "downloaded/jichitai_code_list.xlsx";
   const SHEET_1: &str = "R6.1.1現在の団体";
   const SHEET_2: &str = "R6.1.1政令指定都市";
   ```

4. 実行して`localgov-code.db`を生成

```.rs
$ cargo run
```
