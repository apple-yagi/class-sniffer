use clap::Parser;
use classsniffer::{css, tsx};
use std::fs;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct CliArgs {
    /// Target CSS file (default: stdin)
    #[clap(value_parser)]
    input_file: Vec<String>,
}

fn main() {
    // let cli_args = CliArgs::parse();

    // CSSファイルを読み込み
    // let css_file_path = &cli_args.input_file[1];
    let css_content = fs::read_to_string("testdata/styles.css").expect("Failed to read the CSS file");

    // クラス名を抽出
    let css_classnames = css::extract_classname(&css_content);

    // TSXファイルを読み込み
    let tsx_content = fs::read_to_string("testdata/test.tsx").expect("Failed to read the Tsx file");
    let tsx_classnames = tsx::extract_classname(&tsx_content);

    // TSXファイルに存在するクラス名のうち、CSSファイルに存在しないものを抽出
    let diff_classnames = tsx_classnames
        .iter()
        .filter(|classname| !css_classnames.contains(classname))
        .collect::<Vec<_>>();

    // diff_classnamesが空でない場合、エラーを出力
    if !diff_classnames.is_empty() {
        eprintln!("The following classnames are not defined in the CSS file:");
        for classname in diff_classnames {
            eprintln!("  - {}", classname);
        }
        std::process::exit(1);
    }

    println!("All classnames are defined in the CSS file.");
    std::process::exit(0);
}
