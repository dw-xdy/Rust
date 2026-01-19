use std::fs;
use std::io::{self, Write};
use std::path::Path;

fn main() {
    println!("========================================");
    println!("       极简文件加密/解密工具 (v1.0)      ");
    println!("========================================");

    // 将整个逻辑包在一个大循环里，方便连续处理
    loop {
        println!("\n[ 1.开始处理 | 2.退出 ]");
        let choice = get_input("请输入数字: ");

        if choice == "2" { break; }
        if choice != "1" { continue; }

        let input_path = get_input("1. 请粘贴源文件路径: ").replace('\"', "");

        if !Path::new(&input_path).exists() {
            println!("❌ 错误：找不到文件，请确认路径是否正确。");
            continue;
        }

        let output_path = get_input("2. 请输入保存文件名: ").replace('\"', "");
        let key = get_input("3. 请输入加密钥匙 (Key): ");

        if key.is_empty() {
            println!("❌ 错误：钥匙不能为空！");
            continue;
        }

        println!("正在处理中...");
        match process_file(&input_path, &output_path, &key) {
            Ok(_) => {
                println!("✅ 成功！文件已保存在: {}", output_path);
            }
            Err(e) => println!("❌ 失败: {}", e),
        }
    }

    // 关键：防止双击运行时窗口直接关闭
    println!("\n程序已结束。按【回车键】关闭窗口...");
    let _ = io::stdin().read_line(&mut String::new());
}

fn process_file(input: &str, output: &str, key: &str) -> io::Result<()> {
    let data = fs::read(input)?;
    let key_bytes = key.as_bytes();

    let processed_data: Vec<u8> = data
        .iter()
        .enumerate()
        .map(|(i, &byte)| byte ^ key_bytes[i % key_bytes.len()])
        .collect();

    fs::write(output, processed_data)
}

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("无法读取");
    input.trim().to_string()
}