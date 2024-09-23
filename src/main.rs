use clap::Parser;
use classsniffer::classnames;
use std::fs;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct CliArgs {
    /// Target CSS file (default: stdin)
    #[clap(value_parser)]
    input_file: Vec<String>,
}

fn main() {
    let cli_args = CliArgs::parse();

    // CSSファイルを読み込み
    let css_file_path = &cli_args.input_file[1];
    let css_content = fs::read_to_string(css_file_path).expect("Failed to read the CSS file");

    // クラス名を抽出
    let classnames = classnames::extract(&css_content);

    // 結果を表示
    for classname in classnames {
        println!("{}", classname);
    }
}
