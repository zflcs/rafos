use std::{io::{Result, Write}, fs::File};

fn main() {
    let pwd = std::env::current_dir().unwrap();
    let linker_path = pwd.join("linker.ld");
    println!("cargo:rerun-if-changed=/src/");

    println!("cargo:rustc-link-args=-fnopie -nostartfiles -Bstatic -l:libc.a -T{}", linker_path.display());

    insert_ramfs().unwrap();
}


#[allow(unused)]
fn insert_ramfs() -> Result<()> {
    let pwd = std::env::current_dir().unwrap();
    let mut ancestors = pwd.ancestors();
    let fs_img_path = ancestors.nth(1).unwrap().join("target/riscv64gc-unknown-linux-gnu/release/fs.img");
    let mut f = File::create(pwd.join("src/ramfs.asm")).unwrap();

    writeln!(
        f,
r#".align 3
.section .data.ramfs
.global sramfs
.global eramfs
sramfs:
    .incbin {:?}
eramfs:"#, 
    fs_img_path

    )?;
    Ok(())
}
