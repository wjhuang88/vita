plugins {
    `java-library`
    id("fr.stardustenterprises.rust.importer")
}

description = "vita-libs-native"

java {
    sourceCompatibility = JavaVersion.VERSION_24
    targetCompatibility = JavaVersion.VERSION_24
}

repositories {
    mavenCentral()
}

dependencies {

    rust(project(":native:libvita-sys"))
    api("fr.stardustenterprises:yanl:0.8.1")
    api("com.github.spotbugs:spotbugs-annotations:4.9.6")
    api("io.projectreactor:reactor-core:3.7.9")
    api("com.softwaremill.jox:channels:1.0.1")

    testImplementation(platform("org.junit:junit-bom:5.10.0"))
    testImplementation("org.junit.jupiter:junit-jupiter")
}

rustImport {
    baseDir = "/META-INF/natives"
    layout = "hierarchical"
}

tasks.test {
    useJUnitPlatform()
}