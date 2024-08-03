/// 将字节转换为 人类识别友好
pub fn human_readable_size(bytes: u64) -> String {
    let sizes = ["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB"];
    let mut exp = 0;

    // 当bytes >= 1024即1 KiB时开始处理
    let mut size = bytes;
    while size >= 1024 && exp < (sizes.len() - 1) {
        size >>= 10; // 向右移位10位，相当于除以1024
        exp += 1;
    }

    // 为了准确显示，将最终的size转换为浮点数，并重新计算得到近似的小数形式
    let float_size = bytes as f64 / (1 << (10 * exp)) as f64;
    format!("{:.2} {}", float_size, sizes[exp])
}