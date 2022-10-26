#!/usr/bin/env bash

if [ $# -ne 2 ]; then
	echo usage: $0 destination-directory crate-name
	exit 1
fi

source_dir=$(cd "$(dirname "${BASH_SOURCE[0]}")" &> /dev/null && pwd)
destination_directory="$1"
crate_name="$2"

mkdir -p "$destination_directory"

if [ ! -e "$destination_directory/smithy-rs" ]; then
    pushd "$destination_directory" || exit 1
    echo "Generating project for $crate_name generated in $destination_directory from template"
    echo
	git init .
	git submodule add https://github.com/awslabs/smithy-rs
	cp "$source_dir/build.gradle.kts" build.gradle.kts
	cp "$source_dir/settings.gradle.kts" settings.gradle.kts
	sed "s/%%CRATE_NAME%%/$crate_name/g" "$source_dir/gitignore" >.gitignore
	sed "s/%%CRATE_NAME%%/$crate_name/g" "$source_dir/Cargo.toml" >Cargo.toml
	sed "s/%%CRATE_NAME%%/$crate_name/g" "$source_dir/gradle.properties" >gradle.properties
	mkdir -p model
	cp "$source_dir/model/main.smithy" model/main.smithy
	cp "$source_dir/model/build.gradle.kts" model/build.gradle.kts
	sed "s/%%CRATE_NAME%%/$crate_name/g" "$source_dir/model/smithy-build.json" >model/smithy-build.json
    gradle wrapper
    ./gradlew assemble
    mkdir -p "$crate_name"
    cargo init --lib "$crate_name" --vcs none
    cargo build
    git add .
    git commit -a -m "$crate_name smithy-rs service from template definition"
    popd || exit 1
    echo
    echo "Project directory for $crate_name generated in $destination_directory"
else
    echo "This folder looks to be already initialized. Please remove it before re-running $0"
    exit 1
fi
