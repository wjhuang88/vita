import org.gradle.nativeplatform.platform.internal.DefaultNativePlatform

plugins {
    id("fr.stardustenterprises.rust.wrapper")
}

description = "vita-native-sys"

val libName = "vita"

rust {
    release.set(false)
    cargoInstallTargets.set(true)

    val os = DefaultNativePlatform.getCurrentOperatingSystem()

    targets {
        if (os.isMacOsX) {
            create("macOSAarch64") {
                target = "aarch64-apple-darwin"
                outputName = "lib${libName}64.dylib"
                command = "cargo"
            }
        }

//        create("win64") {
//            target = "x86_64-pc-windows-msvc"
//            outputName = "${libName}64.dll"
//            command = if(os.isWindows) "cargo" else "cargo-xwin"
//        }
//
//        create("linux64") {
//            target = "x86_64-unknown-linux-gnu"
//            outputName = "lib${libName}64.so"
//            command = "cargo-zigbuild"
//        }
//
//        create("linux64Aarch64") {
//            target = "aarch64-unknown-linux-gnu"
//            outputName = "lib${libName}64.so"
//            command = "cargo-zigbuild"
//        }
    }
}