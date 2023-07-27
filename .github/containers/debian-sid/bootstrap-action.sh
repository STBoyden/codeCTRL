#!/usr/bin/env bash

source /etc/os-release

set -xe

echo -e "\nInstalling dependencies for your distro...\n"

case $ID in
    "debian" | "ubuntu" | "elementary")
        export DEBIAN_FRONTEND=noninteractive

        apt update -y
        apt install build-essential libfreetype-dev libfontconfig-dev curl cmake -y
        ;;
    "fedora" | "rocky")
        packages=(freetype-devel expat-devel fontconfig-devel cmake)

        dnf="$(command -v dnf)"

        $dnf update -y

        if [ -f "$(command -v dnf5)" ]; then
            dnf="$(command -v dnf5)"
            sudo "$dnf" group install development-tools -y
        else
            $dnf groupinstall "Development Tools" -y
        fi

        if [[ ! "$ID" == "rocky" ]]; then
            packages+=(g++)
        fi

        if [[ "$ID" == "rocky" && $(rpm -E %rhel) -ge 9 ]]; then
            $dnf --enablerepo=crb --allowerasing install "${packages[@]}" -y
        else
            $dnf install "${packages[@]}" -y
        fi
        ;;
    "centos")
        yum install epel-release -y
        yum update -y
        yum groupinstall "Development Tools" -y

        yum install freetype-devel expat-devel fontconfig-devel cmake3 -y
        ln -s /usr/bin/cmake3 /usr/bin/cmake
        ;;
esac