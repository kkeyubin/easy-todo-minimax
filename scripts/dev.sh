#!/bin/bash
# 一键起 tauri dev：清端口残留 + 起 dev
set -e

PORT=1420

# 1. 杀掉占用 1420 端口的 vite 残留（上次 dev 没关干净会留）
PIDS=$(lsof -ti:$PORT 2>/dev/null || true)
if [ -n "$PIDS" ]; then
  echo "killing stale processes on port $PORT: $PIDS"
  kill -9 $PIDS 2>/dev/null || true
  sleep 1
fi

# 2. 顺手清 easy-sticky 残留
pkill -9 -f "easy-sticky" 2>/dev/null || true
sleep 1

# 3. 起 dev
export PATH="/Users/kyle/.nvm/versions/node/v24.15.0/bin:$PATH"
cd "$(dirname "$0")/.."
exec pnpm tauri:dev
