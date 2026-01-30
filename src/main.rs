use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::path::Path;

fn main() {
    println!("====================================================");
    println!("           欢迎使用文件「隐身」工具 (v1.6)           ");
    println!("      本工具可以对任何文件(小说、图片、视频)加密      ");
    println!("          注意：加密和解密使用的是钥匙(密钥)        ");
    println!("====================================================");

    loop {
        println!("\n--- 功能菜单 ---");
        println!(" [1] 开始 加密 (只需运行一次)");
        println!(" [2] 开始 解密 (只需运行一次)");
        println!(" [3] 退出程序");

        let choice = get_input("\n请选择功能并按回车 (1 或 2 或 3): ");

        if choice == "3" {
            println!("程序已安全退出。");
            break;
        }
        if choice != "1" && choice != "2" {
            println!("⚠️ 输入有误，请输入数字 1 或 2 或 3 喔！");
            continue;
        }

        // 1. 获取输入路径
        println!("\n第一步：告诉我要处理哪个文件");
        println!("(小技巧：你可以直接把文件「拖进」这个黑窗口，路径会自动出现)");
        let input_path = get_input("请输入或拖入文件路径: ").replace('\"', "");

        if !Path::new(&input_path).exists() {
            println!("❌ 找不到文件！请检查你输入的路径是否正确，或者文件名是否写对了。");
            continue;
        }

        // 2. 获取输出路径
        println!("\n第二步：处理后的文件叫什么名字？");
        println!("(例如：如果原文件叫 test.mp4，你可以输入 test_加密.mp4)");
        let output_path = get_input("请输入新的文件名: ").replace('\"', "");

        // 3. 获取钥匙
        println!("\n第三步：设置一把「钥匙」(即密码)");
        println!("⚠️ 重要：解密时必须输入一模一样的钥匙，否则文件无法恢复！");
        let key = get_input("请输入钥匙 (建议是数字或字母): ");
        if key.is_empty() {
            println!("❌ 钥匙不能为空，请重新开始操作。");
            continue;
        }

        // 开始处理
        println!("\n🚀 正在玩命处理中，请稍后...");
        match process_file_with_progress(&input_path, &output_path, &key) {
            Ok(_) => {
                println!("\n✨ 操作成功！✨");
                println!("你的新文件已存放在: {}", output_path);
                println!("提示：如果文件变成乱码或打不开，说明加密成功了！");
                println!("      再用同样的钥匙运行一次，它就会变回原样。");
            }
            Err(e) => {
                println!("\n❌ 发生意外错误: {}", e);
                println!("可能是文件正在被其他程序占用，请关闭后再试。");
            }
        }

        println!("\n----------------------------------------------------");
    }

    // 防止用户没看清结果就窗口关闭
    println!("\n按【回车键】关闭此窗口...");
    let _ = io::stdin().read_line(&mut String::new());
}

fn process_file_with_progress(input: &str, output: &str, key: &str) -> io::Result<()> {
    let input_file = File::open(input)?;
    let total_size = input_file.metadata()?.len();
    let mut reader = BufReader::new(input_file);

    let output_file = File::create(output)?;
    let mut writer = BufWriter::new(output_file);

    let key_bytes = key.as_bytes();
    let mut buffer = [0u8; 8192];
    let mut processed_size = 0u64;
    let mut last_percent = 999;

    while let Ok(n) = reader.read(&mut buffer) {
        if n == 0 {
            break;
        }

        for i in 0..n {
            buffer[i] ^= key_bytes[(processed_size as usize + i) % key_bytes.len()];
        }

        writer.write_all(&buffer[..n])?;
        processed_size += n as u64;

        let percent = (processed_size * 100 / total_size) as usize;
        if percent != last_percent {
            draw_progress_bar(percent);
            last_percent = percent;
        }
    }

    writer.flush()?;
    Ok(())
}

fn draw_progress_bar(percent: usize) {
    let bar_len = 30;
    let filled_len = (percent * bar_len) / 100;
    let mut bar = String::from("进度：[");

    for i in 0..bar_len {
        if i < filled_len {
            bar.push('█');
        } else {
            bar.push('░');
        }
    }
    bar.push_str(&format!("] {}%", percent));

    print!("\r{}", bar);
    io::stdout().flush().unwrap();
}

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
