use std::env;

fn main() {
    let mut builder = cc::Build::new();
    builder.file("cephes-src/stdtr.c");
    builder.file("cephes-src/const.c");
    builder.file("cephes-src/incbet.c");
    builder.file("cephes-src/gamma.c");
    builder.file("cephes-src/mtherr.c");
    builder.file("cephes-src/polevl.c");
    builder.warnings(false);

    // we can't use #[cfg(target_arch = "xtensa")] since build.rs has it set to the hosts target
    if env::var("CARGO_CFG_TARGET_ARCH").unwrap() == "xtensa" {
        builder.no_default_flags(true);
        builder.compiler("xtensa-esp32-elf-gcc");
        builder.archiver("xtensa-esp32-elf-ar");

        // those flags were taken from hacking -DCMAKE_EXPORT_COMPILE_COMMANDS=ON into the ESP-IDF build process
        builder.flag("-mlongcalls");
        builder.flag("-ffunction-sections");
        builder.flag("-fdata-sections");
        builder.flag("-ffunction-sections");
        builder.flag("-fdata-sections");
        builder.flag("-Os");
        builder.flag("-freorder-blocks");
        builder.flag("-fstrict-volatile-bitfields");
        builder.flag("-std=gnu99");
    }

    builder.compile("cephes");
}
