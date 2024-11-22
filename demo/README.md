使用说明
保存脚本：将上述脚本保存为 build_and_run.sh。
赋予执行权限：确保脚本具有执行权限。
sh
深色版本
chmod +x build_and_run.sh
运行脚本：运行脚本以编译和运行所有组件。
sh
深色版本
./build_and_run.sh
注意事项
路径：确保所有文件路径正确，特别是当文件不在同一目录时。
依赖项：确保安装了必要的工具和库，如 wat2wasm、gcc、clang 和 liburing。
参数：根据实际情况修改 python3 "$PYTHON_SCRIPT" 命令中的参数，例如 --program、--language 和 --template。
