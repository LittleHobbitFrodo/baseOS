
//! creates the fs structure or builds the ISO file


use colored::*;
use tomelib as tome;
use std::{fs, str::FromStr};

use tome::*;
use std::path::Path;


static mut REMOVE_OLD_ISO: bool = false;


/// Parameters given to xorriso
/// Append `{iso frame} -o {forged iso}`
const XORRISO_PARAMETERS: &[&'static str] = &["-as", "mkisofs", "-R", "-r", "-J", "-b", "limine-bios-cd.bin",
	"-no-emul-boot", "-boot-load-size", "4", "-boot-info-table", "-hfsplus", "-apm-block-size", "2048",
	"--efi-boot", "limine-uefi-cd.bin", "-efi-boot-part", "--efi-boot-image",
	"--protective-msdos-label"];


const CREATE_ISO_STRUCTURE_CMD_NAME: &'static str = "carve";
const BUILD_ISO_CMD_NAME: &'static str = "seal";


fn main() {
    
    let (actions, archs, switches) = collect_args_with_actions::<Action>();

    unsafe { REMOVE_OLD_ISO = parse_switches(switches) };


    for arch in archs.iter() {

        if unsafe { REMOVE_OLD_ISO } {
            noteln!("removing old iso");
            fs::remove_dir_all(PATH.forged.iso_frame(*arch)).expect("failed to remove the old frame");
        }

        for act in actions.iter() {
            match act {
                Action::Frame => construct_iso_for(*arch),
                Action::Seal => build_iso_for(*arch),
            }
        }
    }


    noteln!("all rituals were considered successful");

}


/// constructs filesystem for the ISO
fn construct_iso_for(arch: Arch) {

    noteln!("carving the ISO rune for the {} target", arch.triplet().blue());

    let frame = PATH.forged.iso_frame_string(arch);
    let kernel = PATH.forged.kernel_string(arch).unwrap();

    let limine_config_path = frame.clone() + "/boot/limine/";
    let efi_boot = frame.clone() + "/EFI/BOOT/";



    if fs::exists(&frame).expect("failed to stat old frame") {
        create_dir_all(&frame);
    }

    if !fs::exists(&kernel).expect("failed to stat kernel") {
        fail!(internal: "kernel for {} is missing", arch.triplet().blue())
    }
    
    //  create directories
    create_dir_all(&efi_boot);
    create_dir_all(&limine_config_path);


    //  copy limine config
    let limine_config = if debug() {
        PATH.limine.config_debug_string()
    } else {
        PATH.limine.config_string()
    };
    copy(limine_config, limine_config_path.clone() + PATH.limine.config_name());


    //  copy the bootloader executable
    copy(PATH.limine.bootloader_for_string(arch).unwrap(), efi_boot.to_string() + arch.bootloader_name());


    //  copy bios and uefi CDs
    copy(PATH.limine.bios_cd_string(), frame.to_string() + PATH.limine.bios_cd_name());
    copy(PATH.limine.uefi_cd_string(), frame.to_string() + PATH.limine.uefi_cd_name());

    //  copy the system file
    copy(PATH.limine.data_dir_string() + "limine-bios.sys", limine_config_path.to_owned() + "limine-bios.sys");


    //  copy the kernel
    copy(kernel, frame.to_string() + "boot/BaseOS.bin");



    //noteln!("The carving of the ISO rune for target {} is complete", arch.triplet().blue());


}

fn build_iso_for(arch: Arch) {

    let frame = PATH.forged.iso_frame_string(arch);

    if !fs::exists(&frame).expect("failed to stat frame") {
        construct_iso_for(arch);
    }

    noteln!("sealing the ISO rune for {}", arch.triplet().blue());

    let tome = TOME_CONFIG.read().expect("failed to acquire lock");

    //  build iso
    let params = xorriso_params(arch);
    match cmd::cmd(&tome.xorriso, Some(params)) {
        Ok(status) => if !status.success() {
            fail!(internal: "xorriso failed")
        },
        Err(e) => fail!(internal: "failed to seal the iso: {e}"),
    }


    let limine = &PATH.limine.cmdline_string();

    //  install limine for bios
    match cmd::cmd(limine, Some(["bios-install".to_string(), PATH.forged.iso_string(arch).unwrap()])) {
        Ok(status) => {
            if !status.success() {
                failln!(internal: "failed to seal the ISO");
            }
        },
        Err(e) => failln!(internal: "failed to seal the ISO: {e}"),
    }


}









/// Appends needed data to the `XORRISO_PARAMETERS` constant
fn xorriso_params(arch: Arch) -> Vec<String> {
    let mut args: Vec<String> = XORRISO_PARAMETERS.iter().map(|x| x.to_string()).collect();
    args.push(PATH.forged.iso_frame_string(arch));
    args.push("-o".to_string());
    args.push(PATH.forged.iso_string(arch).unwrap());
    args
}



/// Recursively creates directory
fn create_dir_all<P>(path: P)
where P: AsRef<Path>, String: From<P> {
    if let Err(_) = fs::create_dir_all(&path) {
        failln!(internal: "failed to create {} directory", String::from(path));
    }
}

fn copy<S, P>(from: S, to: P)
where S: AsRef<Path>, String: From<S>, P: AsRef<Path>, String: From<P> {
    if let Err(_) = fs::copy(&from, &to) {
        failln!(internal: "failed to copy {} to {}", String::from(from), String::from(to));
    }
}


/// Parses switches
/// - Returns `(remove)`
fn parse_switches(switches: Vec<String>) -> bool {

    let mut remove = false;

    for switch in switches {
        match switch.as_str() {
            "-remove" => remove = true,
            _ => failln!(internal: "unknown switch {}", switch.blue()),
        }
    }

    remove
}

#[derive(Copy, Clone, Debug)]
enum Action {
    /// Creates the ISO structure
    Frame,
    /// Builds the ISo file from the structure
    /// - fails if the structure is not found
    Seal,
}

impl FromStr for Action {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            CREATE_ISO_STRUCTURE_CMD_NAME => Ok(Self::Frame),
            BUILD_ISO_CMD_NAME => Ok(Self::Seal),
            _ => Err(()),
        }
    }
}
