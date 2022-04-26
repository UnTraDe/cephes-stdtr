use std::env;

fn main() {
    let mut builder = cc::Build::new();

    if cfg!(feature = "single") {
        builder.file("cephes-src/single/stdtrf.c");
        builder.file("cephes-src/single/constf.c");
        builder.file("cephes-src/single/incbetf.c");
        builder.file("cephes-src/single/gammaf.c");
        builder.file("cephes-src/single/mtherr.c");
        builder.file("cephes-src/single/polevlf.c");
        builder.warnings(false);
    } else {
        builder.file("cephes-src/double/stdtr.c");
        builder.file("cephes-src/double/const.c");
        builder.file("cephes-src/double/incbet.c");
        builder.file("cephes-src/double/gamma.c");
        builder.file("cephes-src/double/mtherr.c");
        builder.file("cephes-src/double/polevl.c");
        builder.warnings(false);
    }

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
