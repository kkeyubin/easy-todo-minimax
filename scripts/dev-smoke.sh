#!/bin/bash
# 后台跑 tauri dev 20s，捕日志，清理
export PATH="/Users/kyle/.nvm/versions/node/v24.15.0/bin:$PATH"
cd /Users/kyle/Codes/easy-todo-minimax
./node_modules/.bin/tauri dev > /tmp/tauri-dev.log 2>&1 &
PID=$!
sleep 25
if ps -p $PID > /dev/null; then
  STATUS="still running"
else
  STATUS="exited"
fi
echo "--- tauri dev $STATUS (pid=$PID) ---"
echo "--- log ---"
cat /tmp/tauri-dev.log
echo "--- cleanup ---"
kill $PID 2>/dev/null
pkill -f "easy-sticky" 2>/dev/null
sleep 2
echo "--- done ---"
