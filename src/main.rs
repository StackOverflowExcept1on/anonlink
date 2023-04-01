#![feature(exit_status_error)]

use iced_x86::{Code, Decoder, DecoderOptions};
use pelite::pe64::{Pe, PeFile};
use std::io::{Read, Seek, Write};
use std::os::windows::process::CommandExt;
use std::path::PathBuf;
use std::process::Command;
use std::{env, fs, io};

use ext::*;
pub use result::*;

mod ext;
mod pdb;
mod result;

fn locate_link() -> io::Result<PathBuf> {
    let mut path: PathBuf = env::var_os("ProgramFiles(x86)")
        .ok_or_else(|| io::Error::from(io::ErrorKind::NotFound))?
        .into();

    path.extend(&["Microsoft Visual Studio", "Installer", "vswhere.exe"]);

    let output = Command::new(path)
        .args([
            "-latest",
            "-requires",
            "Microsoft.VisualStudio.Component.VC.Tools.x86.x64",
            "-find",
            r"VC\Auxiliary\Build\vcvars64.bat",
        ])
        .output()?
        .exit_result()?;

    let vcvars64 = output.stdout.read_process_line()?;

    let output = Command::new("cmd")
        .raw_arg(&format!("/C \"call \"{vcvars64}\" > NUL && where link\""))
        .output()?
        .exit_result()?;

    Ok(output.stdout.read_process_line()?.into())
}

fn main() -> Result<()> {
    let path = locate_link()?;
    println!("linker path: {}", path.display());

    let content = fs::read(&path)?;
    let pe_file = PeFile::from_bytes(&content)?;

    let backup_path = path
        .parent()
        .expect("failed to get parent path")
        .join("link_backup.exe");

    if backup_path.exists() {
        println!(
            "Found backup file: {}, skipping patching",
            backup_path.display()
        );
        return Ok(());
    }

    fs::copy(&path, &backup_path)?;

    let section = pe_file
        .section_headers()
        .by_name(".text")
        .expect("failed to find .text section");

    let symbol_resolver = pdb::symbol_table::download_symbols(pe_file)?;

    let section_range = section.virtual_range();
    let build_image_address = *symbol_resolver
        .symbol2rva("?BuildImage@IMAGE@@QEAAHXZ")
        .expect("failed to locate symbol IMAGE::BuildImage(...)");

    let ip = build_image_address;
    let len = section_range.end - build_image_address as u32;

    let bytes = pe_file.derva_slice::<u8>(build_image_address as u32, len as usize)?;

    let mut decoder = Decoder::try_with_ip(64, bytes, ip, DecoderOptions::NONE)?;

    let cb_build_prod_id_block = *symbol_resolver
        .symbol2rva("?CbBuildProdidBlock@IMAGE@@AEAAKPEAPEAX@Z")
        .expect("failed to locate symbol IMAGE::CbBuildProdidBlock(...)");

    //call IMAGE::CbBuildProdidBlock()
    let inst = decoder
        .iter()
        .find(|inst| inst.is_call_near() && inst.near_branch_target() == cb_build_prod_id_block)
        .expect("failed to find call instruction");

    let address = pe_file.optional_header().ImageBase + inst.ip();
    println!("Found call instruction at address {address:02X?}");

    //add reg32, reg32
    let inst = decoder
        .iter()
        .find(|inst| inst.code() == Code::Add_r32_rm32)
        .expect("failed to find add instruction");

    let address = pe_file.optional_header().ImageBase + inst.ip();
    println!("Found add instruction at address {address:02X?}");

    let raw_offset = inst.ip() - section_range.start as u64;
    let file_offset = section.PointerToRawData as u64 + raw_offset;

    let mut file = fs::File::options().read(true).write(true).open(path)?;
    file.seek(io::SeekFrom::Start(file_offset))?;

    let mut buf = vec![0; inst.len()];
    file.read_exact(&mut buf)?;

    let patched = vec![0x90; inst.len()];
    println!("Patching bytes {buf:02X?} => {patched:02X?}");
    file.seek(io::SeekFrom::Start(file_offset))?;
    file.write_all(&patched)?;

    Ok(())
}
