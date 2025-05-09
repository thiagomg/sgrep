#!/bin/bash

f_pushd() {

    pushd "$@" &> /dev/null
}


f_popd() {

    popd &> /dev/null
}

target="x86_64-unknown-linux-gnu"

oses=(
    "22.04"
    "24.04"
)

rm -rf ./dist/

for os in "${oses[@]}"; do
    full_target="ubuntu-${os}"
    mkdir -p "dist/${full_target}"
    cp "target-${os}/${target}/release/sgrep" "dist/${full_target}"
    
    f_pushd "dist/${full_target}"
    
    version=$(./sgrep --version | awk '{print $2}')
    echo "sgrep ${version} in ${full_target}"
    bzip2 sgrep
    mv "sgrep.bz2" "../sgrep-${version}_${full_target}.bz2"
    cd ..
    rmdir "${full_target}"
    
    f_popd

done
