#!/usr/bin/env bash

for dir in ./*; do
    [ ! -d "$dir" ] && continue
    distro=$(basename "$dir")

    echo "Building container for '${distro}'..."

    (
        cd "$distro" || (echo "Could not cd into '${distro}'" && exit)

        package_name="ghcr.io/stboyden/codectrl-pkg/${distro}"
        podman build . -t "$package_name" \
            --label "org.opencontainers.image.source=https://github.com/stboyden/codectrl" \
            --label "org.opencontainers.image.description=Build container for ${distro}" \
            --label "org.opencontainers.image.licenses=MIT" \
            ||\
            (echo "Could not build container for '${distro}'" && exit)
        podman push "$package_name" ||\
            (echo "Could not push container for '${distro}'" && exit)
    )

    echo -e "\n"
done