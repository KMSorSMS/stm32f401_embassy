#!/bin/bash
# 测试有两个Executor 4个Task的情况
# 先运行测试：cargo run --bin test_2e_4t,将结果重定向到tmp文件，再从里面选取出执行时间
# 比如task_1 execute time:{}ms这种信息
# cargo run --bin test_2e_4t > tmp.yaml --release
# cat tmp | grep "task_1 execute time"
# cargo run --bin test_2e_4t --release > tmp.yaml
# 在后台运行cargo命令
run_test() {
    local test_name=$1
    rm tmp.yaml
    cargo run --bin "$test_name" --release > tmp.yaml &
    # 记录上一次文件大小
    PREV_SIZE=0

    # 检查文件大小的时间间隔（秒）
    INTERVAL=2

    # 获取cargo命令的PID
    CARGO_PID=$!
    echo "Cargo PID: $CARGO_PID"
    # 最大等待时间（秒）
    MAX_WAIT=3
    WAITED=0
    # # 查找并终止probe-rs run进程
    # sleep 2
    # PROBE_RS_PID=$(pgrep -f "probe-rs")
    # echo "Probe-rs PID: $PROBE_RS_PID"
    while true; do
    # 获取当前文件大小
    CURRENT_SIZE=$(stat -c %s tmp.yaml 2>/dev/null || echo "0")
        # 只有当前文件大小非0才监测
    if [ "$CURRENT_SIZE" -ne 0 ]; then
    # 检查文件大小是否有变化
    if [ "$CURRENT_SIZE" -eq "$PREV_SIZE" ]; then
        if [ "$WAITED" -ge "$MAX_WAIT" ]; then
        # 文件大小在指定时间内没有变化，终止cargo命令
        kill $CARGO_PID
        echo "Cargo command terminated due to inactivity."
        break
        else
        # 等待更长时间
        ((WAITED+=INTERVAL))
        fi
    else
        # 文件大小有变化，重置等待时间
        WAITED=0
    fi
    PREV_SIZE=$CURRENT_SIZE
    fi
    sleep $INTERVAL
    done

    # 继续执行脚本的其他部分
    # 得到程序执行时间的信息
    cat tmp.yaml | grep -E "task(_[0-9]+)* execute time"
}
clear
echo "=============Start testing=============" > record.yml

# 定义一个数组，包含所有测试
tests=(
"test_2e_4t" 
"test_3e_6t" 
"test_2e_8t" 
"test_2e_20t"
)

# 循环遍历数组，执行测试
for test in "${tests[@]}"; do
    echo "=============${test}=============" >> record.yml
    run_test "$test" >> record.yml
    echo -e "=============${test} done=============\n" >> record.yml
    sleep 1
done

# 为了方便画出表格图，单独把时间信息提取出来，放到time.txt文件中