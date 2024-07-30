use polars::prelude::*;
use std::fs::File;
use std::path::Path;

fn main() -> Result<(),Box<dyn std::error::Error>> {
    println!("cargo:rustc-link-search=libsrp6");
    // CSVファイルのパスを指定
    let file_path = Path::new("../input/titanic3.csv");

    // データを読み込む
    let df = read_csv(file_path)?;

    ans9(df);

    Ok(())
}

// CSVファイルを読み込む関数
fn read_csv(file_path: &Path) -> Result<DataFrame,Box<dyn std::error::Error>> {
    let file = File::open(&file_path)?;
    let df = CsvReader::new(file)
        .infer_schema(Some(100)) // 最初の100行からスキーマを推測
        .has_header(true)        // ヘッダー行があることを指定
        .finish()?;              // DataFrameを生成

    Ok(df)
}

fn ans1(df: DataFrame){
    println!("{:?}",df.head(None));
}
fn ans2(df: DataFrame){
    println!("{:?}",df.tail(None));
}
fn ans3(df: DataFrame){
    println!("{:?}",df.shape());
}
fn ans4(df: DataFrame){
    println!("{:?}",df.sort(&["fare"], false));
}
fn ans5(df: DataFrame){
    println!("{:?}",df.schema());
}
fn ans6(df: DataFrame){
    println!("{:?}",df.height());
}
fn ans7(df: DataFrame){
    println!("{:?}",df.describe(None));
}
fn ans8(df: DataFrame){
    println!("{:?}",df.select(["sex"]));
}
fn ans9(df: DataFrame){
    println!("{:?}", df.get_columns());
}