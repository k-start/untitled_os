use bootloader_locator::locate_bootloader;
use std::env;
use std::path::Path;
use std::process::{Command, Stdio};

pub fn main() {
    let args: Vec<String> = env::args().collect();

    let bootloader_manifest = locate_bootloader("bootloader").unwrap();

    let kernel_binary = Path::new(&args[1]).canonicalize().unwrap();
    // the path to the root of this crate, set by cargo
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    // we know that the kernel lives in the parent directory
    let kernel_dir = manifest_dir.parent().unwrap();

    let kernel_manifest = kernel_dir.join("Cargo.toml");
    // use the same target folder for building the bootloader
    let target_dir = kernel_dir.join("target");
    // place the resulting disk image next to our kernel binary
    let out_dir = kernel_binary.parent().unwrap();

    // create a new build command; use the `CARGO` environment variable to
    // also support non-standard cargo versions
    let mut build_cmd = Command::new(env!("CARGO"));

    // pass the arguments
    build_cmd.arg("builder");
    build_cmd.arg("--kernel-manifest").arg(&kernel_manifest);
    build_cmd.arg("--kernel-binary").arg(&kernel_binary);
    build_cmd.arg("--target-dir").arg(&target_dir);
    build_cmd.arg("--out-dir").arg(&out_dir);

    // set the working directory
    let bootloader_dir = bootloader_manifest.parent().unwrap();
    build_cmd.current_dir(&bootloader_dir);

    // run the command
    let exit_status = build_cmd.status().unwrap();
    if !exit_status.success() {
        panic!("bootloader build failed");
    }

    let mut qemu_args = vec![
        // "-boot",
        // "menu=on",
        "-drive",
    ];

    if args.len() >= 3 && args[2] == "uefi" {
        let mut uefi_args = vec![
            "format=raw,file=target/x86_64-os/debug/boot-uefi-untitled_os.img,bus=0",
            "-bios",
            "OVMF-pure-efi.fd",
        ];
        qemu_args.append(&mut uefi_args);
    } else {
        let mut bios_args =
            vec!["format=raw,file=target/x86_64-os/debug/boot-bios-untitled_os.img,bus=0"];
        qemu_args.append(&mut bios_args);
    }

    let mut more_args = vec![
        "-drive",
        "format=raw,file=fat32.img,bus=1",
        "-serial",
        "stdio",
    ];
    qemu_args.append(&mut more_args);

    Command::new("qemu-system-x86_64")
        .args(qemu_args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("failed to execute process");
}
