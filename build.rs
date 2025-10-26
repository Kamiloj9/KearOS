use std::{env, fs, path::PathBuf};

fn main() {
    let target = env::var("TARGET").unwrap_or_default();
    if target != "thumbv8m.main-none-eabi" {
        println!("cargo:warning=Skipping Pico SDK build for host target: {}", target);
        println!("cargo:rerun-if-changed=native/CMakeLists.txt");
        println!("cargo:rerun-if-changed=native/cshim.c");
        println!("cargo:rerun-if-env-changed=PICO_SDK_PATH");
        return;
    }

    // track file changes for rebuilds
    println!("cargo:rerun-if-changed=native/CMakeLists.txt");
    println!("cargo:rerun-if-changed=native/cshim.c");
    println!("cargo:rerun-if-changed=native/stubs.c");
    println!("cargo:rerun-if-env-changed=PICO_SDK_PATH");

    // === Step 1: Build Pico SDK shim via CMake ===
    let dst = cmake::Config::new("native")
        .define("PICO_PLATFORM", "rp2350-arm-s")
        .define("PICO_BOARD", "pico2")
        .define("CMAKE_SYSTEM_NAME", "Generic")
        .define("CMAKE_SYSTEM_PROCESSOR", "arm")
        .define("CMAKE_C_COMPILER", "arm-none-eabi-gcc")
        .define("CMAKE_CXX_COMPILER", "arm-none-eabi-g++")
        .define("CMAKE_ASM_COMPILER", "arm-none-eabi-gcc")
        .define("PICO_STDIO_USB", "0")
        .define("PICO_STDIO_UART", "0")
        .cflag("-ffunction-sections -fdata-sections -mthumb -march=armv8-m.main")
        .cxxflag("-ffunction-sections -fdata-sections -mthumb -march=armv8-m.main")
        .asmflag("-ffunction-sections -fdata-sections -mthumb -march=armv8-m.main")
        .build();

    // === Step 2: Link generated static shim ===
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=picosdkshim");

    // === Step 3: Copy Pico SDK’s boot2 blob into OUT_DIR ===
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let build_dir = out_dir.join("build");

    // The SDK produces this automatically during CMake build
    let boot2_src = build_dir
        .join("pico-sdk")
        .join("src")
        .join("rp2350")
        .join("boot_stage2")
        .join("bs2_default.bin");

    let boot2_dst = out_dir.join("boot2.bin");

    if boot2_src.exists() {
        fs::copy(&boot2_src, &boot2_dst)
            .expect(&format!("Failed to copy boot2 blob from {}", boot2_src.display()));
        println!("cargo:warning=Copied boot2 blob to {}", boot2_dst.display());
    } else {
        println!("cargo:warning=boot2 blob not found at {}", boot2_src.display());
        println!("cargo:warning=Did the Pico SDK build succeed?");
    }

    // trigger rebuild if the boot2 blob changes
    println!("cargo:rerun-if-changed={}", boot2_src.display());
}
