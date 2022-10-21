rootProject.name = "pokemon-service"

includeBuild("smithy-rs")
include(":model")

plugins {
    id("software.amazon.smithy").version("0.6.0").apply(false)
    kotlin("jvm").version("1.6.10").apply(false)
}
