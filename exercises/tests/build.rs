//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

fn main() {
    // In tests7, we should set up an environment variable
    // called `TEST_FOO`. Print in the standard output to let
    // Cargo do it.
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs(); // What's the use of this timestamp here?
    // 告诉 Cargo 设置环境变量 TEST_FOO
    println!("cargo:rustc-env=TEST_FOO={}", timestamp); // ✅ 正确的配置出口

    // (可选项) 如果依赖项发生变化时需要重新运行 build.rs
    println!("cargo:rerun-if-env-changed=TEST_FOO");

    // In tests8, we should enable "pass" feature to make the
    // testcase return early. Fill in the command to tell
    // Cargo about that.
    // 新功能：启用 `pass` feature
    println!("cargo:rustc-cfg=feature=\"pass\""); // ✅ 启用条件编译

    // 配置重新构建触发条件
    println!("cargo:rerun-if-env-changed=TEST_FOO");
}
