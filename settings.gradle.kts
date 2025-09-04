pluginManagement {
    plugins {
        id("fr.stardustenterprises.rust.wrapper").version("3.2.5").apply(false)
        id("fr.stardustenterprises.rust.importer").version("3.2.5").apply(false)
    }
    repositories {
        mavenCentral()
        gradlePluginPortal()
    }
}

rootProject.name = "vita"

include("libs:vita-lib-native")
include("native:libvita-sys")