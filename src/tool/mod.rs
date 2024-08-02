use std::time::{SystemTime, UNIX_EPOCH};

pub use decompress_compress_str_tool::DecompressCompressStrTool;
pub use folder_tree_tool::FolderTreeTool;
pub use json_format_tool::JsonFormatTool;

mod folder_tree_tool;
mod decompress_compress_str_tool;
mod json_format_tool;

/// 相对 1970-01-01 00:00:00 UTC 过了多少秒
pub fn get_seconds_since_epoch() -> u64 {
    let now = SystemTime::now();  // 获取当前系统时间
    let since_the_epoch = now.duration_since(UNIX_EPOCH)
        .expect("Time went backwards");  // 计算从UNIX纪元到现在的时间差
    let seconds_since_epoch = since_the_epoch.as_secs();
    seconds_since_epoch
}