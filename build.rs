use std::process::Command;

fn main() {
    // 检测主机架构
    let host_arch = String::from_utf8(
        Command::new("uname")
            .arg("-m")
            .output()
            .expect("Failed to run uname")
            .stdout,
    )
    .expect("Invalid UTF-8 output from uname")
    .trim()
    .to_string();

    let host_arch = match host_arch.as_str() {
        s if s.starts_with("i") && s.ends_with("86") => "i686",
        s if s.starts_with("armv") => "arm",
        "x86_64" => "x86_64",
        _ => panic!("Unsupported architecture: {}", host_arch),
    };

    // 根据架构设置 GenICam 库子目录
    let genicam_lib_subdir = match host_arch {
        "x86_64" => "Linux64_x64",
        "i686" => "Linux32_i86",
        "arm" => "Linux32_ARM",
        _ => unreachable!(),
    };

    // 设置动态库路径
    let lib_dir = "/opt/HuarayTech/MVviewer/lib";
    let genicam_lib_dir = format!(
        "/opt/HuarayTech/MVviewer/lib/GenICam/bin/{}",
        genicam_lib_subdir
    );

    // 链接动态库
    println!("cargo:rustc-link-search=native={}", lib_dir);
    println!("cargo:rustc-link-search=native={}", genicam_lib_dir);
    println!("cargo:rustc-link-lib=dylib=MVSDK");
    println!("cargo:rustc-link-lib=dylib=rt");
    println!("cargo:rustc-link-lib=dylib=pthread");
    println!("cargo:rustc-link-lib=dylib=ImageSave");

    // 嵌入运行时库路径（相当于 -rpath）
    println!("cargo:rustc-link-arg=-Wl,-rpath,{}", lib_dir);
    println!("cargo:rustc-link-arg=-Wl,-rpath,{}", genicam_lib_dir);
}
