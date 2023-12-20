use std::{io::{Result, Write}, fs::File};

fn main() {
    let pwd = std::env::current_dir().unwrap();
    let linker_path = pwd.join("linker.ld");
    println!("cargo:rerun-if-changed=/src/");

    println!("cargo:rustc-link-args=-fnopie -nostartfiles -Bstatic -l:libc.a -T{}", linker_path.display());
    let mut ancestors = pwd.ancestors();
    let workspace_path = ancestors.nth(1).unwrap();
    let src = workspace_path.join("rafos-apps/src/bin/");
    let target = workspace_path.join("target/riscv64gc-unknown-linux-gnu/release/");
    pack_fat32::pack_fat32("fs.img", src.to_str().unwrap(), target.to_str().unwrap());
    insert_ramfs().unwrap();
}


#[allow(unused)]
fn insert_ramfs() -> Result<()> {
    let pwd = std::env::current_dir().unwrap();
    let fs_img_path = pwd.join("fs.img");
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

#[allow(unused)]
pub mod pack_fat32 {
    use fatfs::{format_volume, FileSystem, FormatVolumeOptions, FsOptions, StdIoWrapper, Write};
    use fscommon::BufStream;
    use std::{
        fs::{self, DirEntry, File},
        io::{self, Read},
    };


    fn traverse_dir(file: DirEntry, target_dir: String, names: &mut Vec<String>) {
        let file_name = file.file_name().into_string().unwrap();
        if file.path().is_dir() {
            println!("dir: {}", file.file_name().into_string().unwrap());
            names.push(format!("{}{}/", target_dir, file_name));
            for inner_entry in fs::read_dir(file.path()).unwrap() {
                traverse_dir(
                    inner_entry.unwrap(),
                    format!("{}{}/", target_dir, file_name),
                    names,
                );
            }
        } else {
            names.push(format!("{}{}", target_dir, file_name));
        }
    }

    fn traverse_fat_dir<'a>(
        root: &fatfs::Dir<
            '_,
            fatfs::StdIoWrapper<fscommon::BufStream<std::fs::File>>,
            fatfs::ChronoTimeProvider,
            fatfs::LossyOemCpConverter,
        >,
        file: fatfs::DirEntry<
            '_,
            fatfs::StdIoWrapper<fscommon::BufStream<std::fs::File>>,
            fatfs::ChronoTimeProvider,
            fatfs::LossyOemCpConverter,
        >,
        dir_now: String,
    ) {
        if dir_now != "" {
            print!("\t");
        }
        if !file.is_dir() {
            println!("{}", file.file_name());
        } else {
            let inner_dir = dir_now + file.file_name().as_str() + "/";
            println!("{}", &inner_dir);
            for dir_entry in root.open_dir(inner_dir.as_str()).unwrap().iter() {
                let file = dir_entry.unwrap();
                // Escape hidden files or directories.
                if !file.file_name().starts_with(".") {
                    traverse_fat_dir(root, file, inner_dir.clone());
                }
            }
        }
    }

    fn create_new_fs(name: &str) -> io::Result<()> {
        let img_file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&name)
            .unwrap();
        img_file
            .set_len(8 * 2048 * 512)
            .unwrap();
        let buf_file = BufStream::new(img_file);
        format_volume(
            &mut StdIoWrapper::from(buf_file),
            FormatVolumeOptions::new(),
        )
        .unwrap();
        Ok(())
    }

    pub fn pack_fat32(name: &str, src: &str, target: &str) {
        println!("FAT32 image");
        create_new_fs(name).unwrap();
        let mut user_apps: Vec<String> = vec![];
        for dir_entry in fs::read_dir(src).unwrap() {
            let file = dir_entry.unwrap();
            traverse_dir(file, String::from(""), &mut user_apps);
        }

        let img_file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(name)
            .unwrap();
        let buf_file = BufStream::new(img_file);
        let options = FsOptions::new().update_accessed_date(true);
        let fs = FileSystem::new(buf_file, options).unwrap();
        let root = fs.root_dir();

        user_apps.sort();

        for app in user_apps {
            if app.ends_with("/") {
                println!("[User dir] {}", app.as_str());
                root.create_dir(app.as_str()).unwrap();
            } else {
                let app = app.trim_end_matches(".rs");
                println!("{}", format!("{}{}", target, app));
                let mut origin_file = File::open(format!("{}{}", target, app)).unwrap();
                let mut all_data: Vec<u8> = Vec::new();
                origin_file.read_to_end(&mut all_data).unwrap();
                println!("[User app] {}", app);
                let mut file_in_fs = root.create_file(app).unwrap();
                file_in_fs.write_all(all_data.as_slice()).unwrap();
            }
        }

        println!("\n------------ Listing apps in FAT32 image ------------");
        // List user apps in fat32.
        for dir_entry in root.iter() {
            traverse_fat_dir(&root, dir_entry.unwrap(), String::from(""));
        }
    }
}
