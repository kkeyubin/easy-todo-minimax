// 防止额外控制台窗口在 Windows 上弹出，macOS 上不生效
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    easy_sticky_lib::run()
}
