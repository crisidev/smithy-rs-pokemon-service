val smithyVersion: String by project

buildscript {
    repositories { mavenLocal() }

    val serverCodegenVersion: String by project
    dependencies {
        classpath("software.amazon.smithy.rust.codegen.server.smithy:codegen-server:$serverCodegenVersion")
    }
}

plugins { id("software.amazon.smithy") }

dependencies {
    implementation("software.amazon.smithy:smithy-aws-traits:$smithyVersion")
    implementation("software.amazon.smithy:smithy-model:$smithyVersion")
}

smithy { outputDirectory = buildDir.resolve("codegen") }

tasks {
    val srcDir = projectDir.resolve("../")
    val serverSdkCrateName: String by project
    val copyServerCrate =
            create<Copy>("copyServerCrate") {
                from("$buildDir/codegen/$serverSdkCrateName/rust-server-codegen")
                into("$srcDir/$serverSdkCrateName")
            }

    val clientSdkCrateName: String by project
    val copyClientCrate =
            create<Copy>("copyClientCrate") {
                from("$buildDir/codegen/$clientSdkCrateName/rust-codegen")
                into("$srcDir/$clientSdkCrateName")
            }

    val generateWorkspace = create<Task>("generateWorkspace")

    getByName("assemble").dependsOn("smithyBuildJar")
    getByName("assemble").finalizedBy(copyServerCrate, copyClientCrate, generateWorkspace)
}
