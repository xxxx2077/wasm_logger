#!/bin/bash

# 设置变量
WASM_SOURCE="log_writer.wat"
WASM_OUTPUT="log_writer.wasm"
HOST_C_SOURCE="host_io_uring.c"
HOST_C_OUTPUT="host_io_uring"
EBPF_SOURCE="logReducer.c"
EBPF_OUTPUT="logReducer.o"
PYTHON_SCRIPT="logReducer.py"
LIBURING_PATH="/usr/lib/x86_64-linux-gnu"  # 根据实际路径修改
INCLUDE_PATH="/usr/include"  # 根据实际路径修改

# 编译 Wasm 模块
echo "Compiling Wasm module..."
wat2wasm "$WASM_SOURCE" -o "$WASM_OUTPUT"
if [ $? -ne 0 ]; then
    echo "Failed to compile Wasm module."
    exit 1
fi

# 编译宿主环境 C 代码
echo "Compiling host environment C code..."
gcc -o "$HOST_C_OUTPUT" "$HOST_C_SOURCE" -I"$INCLUDE_PATH" -L"$LIBURING_PATH" -liouring -Wl,-rpath,$LIBURING_PATH
if [ $? -ne 0 ]; then
    echo "Failed to compile host environment C code."
    exit 1
fi

# 编译 eBPF 程序
echo "Compiling eBPF program..."
clang -O2 -target bpf -c "$EBPF_SOURCE" -o "$EBPF_OUTPUT"
if [ $? -ne 0 ]; then
    echo "Failed to compile eBPF program."
    exit 1
fi

# 运行用户空间脚本
echo "Running user space script..."
python3 "$PYTHON_SCRIPT" --program your_program_name --language python --template "your_log_template"
if [ $? -ne 0 ]; then
    echo "Failed to run user space script."
    exit 1
fi

echo "All steps completed successfully."