use classsniffer::classnames;
use std::fs;

fn main() {
    // CSSファイルを読み込み
    let css_file_path = "testdata/styles.css"; // ここにCSSファイルのパスを指定
    let css_content = fs::read_to_string(css_file_path).expect("Failed to read the CSS file");

    // クラス名を抽出
    let classnames = classnames::extract(&css_content);

    // 結果を表示
    for classname in classnames {
        println!("{}", classname);
    }
}
