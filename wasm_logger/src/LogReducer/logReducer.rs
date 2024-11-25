use std::process::Command;
use std::error::Error;

fn logReducerRun() -> Result<(), Box<dyn Error>> {
    // 定义 Python 脚本的路径和参数
    let python_script = "./logReducer.py";
    let program = "hello";
    let language = "rust";
    let template = "[2024-11-25T16:29:37.123Z] [INFO] This is a log message";

    // 创建 Command 对象并设置参数
    let mut cmd = Command::new("python3");
    cmd.arg(python_script)
       .arg("--program").arg(program)
       .arg("--language").arg(language)
       .arg("--template").arg(template);

    // 执行命令并捕获输出
    let output = cmd.output()?;

    // 检查命令是否成功执行
    if output.status.success() {
        // 输出标准输出
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("Standard Output:\n{}", stdout);

        // 输出标准错误
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("Standard Error:\n{}", stderr);
    } else {
        // 输出错误信息
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Command failed with error:\n{}", stderr);
    }

    Ok(())
}