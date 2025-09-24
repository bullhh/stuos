use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    // 获取目标三元组
    let target = env::var("TARGET").unwrap();
    
    // 只有在aarch64目标上才编译汇编代码
    if target.contains("aarch64") {
        let out_dir = env::var("OUT_DIR").unwrap();
        let src_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        
        // 汇编文件路径
        let asm_file = Path::new(&src_dir).join("src").join("boot.s");
        let obj_file = Path::new(&out_dir).join("boot.o");
        
        // 编译汇编文件
        let status = Command::new("aarch64-linux-gnu-as")
            .arg("-o")
            .arg(&obj_file)
            .arg(&asm_file)
            .status()
            .expect("Failed to compile assembly file");
            
        if !status.success() {
            panic!("Failed to compile assembly file");
        }
        
        // 创建静态库
        let lib_file = Path::new(&out_dir).join("libboot.a");
        let status = Command::new("aarch64-linux-gnu-ar")
            .arg("rcs")
            .arg(&lib_file)
            .arg(&obj_file)
            .status()
            .expect("Failed to create static library");
            
        if !status.success() {
            panic!("Failed to create static library");
        }
        
        // 链接汇编对象文件
        println!("cargo:rustc-link-search=native={}", out_dir);
        println!("cargo:rustc-link-lib=static=boot");
    }
}