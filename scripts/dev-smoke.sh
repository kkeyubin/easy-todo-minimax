#!/bin/bash
# 后台跑 tauri dev 90s（让 cargo 完整编完），捕日志，清理
export PATH="/Users/kyle/.nvm/versions/node/v24.15.0/bin:$PATH"
cd /Users/kyle/Codes/easy-todo-minimax
./node_modules/.bin/tauri dev > /tmp/tauri-dev-full.log 2>&1 &
PID=$!
echo "tauri dev pid: $PID"
sleep 90
if ps -p $PID > /dev/null; then
  STATUS="still running"
else
  STATUS="exited"
fi
echo "--- tauri dev $STATUS (pid=$PID) ---"
echo "--- log (last 60 lines) ---"
tail -60 /tmp/tauri-dev-full.log
echo "--- log size: $(wc -l < /tmp/tauri-dev-full.log) lines ---"
echo "--- cleanup ---"
kill $PID 2>/dev/null
pkill -f "easy-sticky" 2>/dev/null
sleep 2
echo "--- done ---"
