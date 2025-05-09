#!/bin/bash

f_pushd() {
    
    pushd "$@" &> /dev/null
}


f_popd() {

    popd &> /dev/null
}

targets=(
    "x86_64-unknown-linux-gnu"
    "x86_64-unknown-linux-musl"
)

for target in "${targets[@]}"; do
    echo "Building for x86_64-unknown-linux-gnu"
    cargo build --release --target=x86_64-unknown-linux-gnu
done

rm -rf ./dist/
for target in "${targets[@]}"; do
    mkdir -p "dist/${target}"
    cp "target/${target}/release/sgrep" "dist/${target}"
    
    f_pushd "dist/${target}"
    
    version=$(./sgrep --version | awk '{print $2}')
    echo "sgrep ${version} in ${target}"
    bzip2 sgrep
    mv "sgrep.bz2" "../sgrep-${version}_${target}.bz2"
    cd ..
    rmdir "${target}"
    
    f_popd
done

