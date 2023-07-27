#!/usr/bin/env bash

source /etc/os-release

if [ $(id -u) -ne 0 ]; then
    echo "Please run this script as root"
    exit 1
fi

echo -e "\nInstalling dependencies for your distro...\n"

case $ID in
    "debian" | "ubuntu" | "elementary")
        export DEBIAN_FRONTEND=noninteractive

        apt update -y
        apt install build-essential libfreetype-dev libfontconfig-dev
        ;;
    "fedora" | "rocky")
        packages=(freetype-devel expat-devel fontconfig-devel)

        dnf="$(command -v dnf)"

        $dnf update -y

        if [ -f "$(command -v dnf5)" ]; then
            dnf="$(command -v dnf5)"
            sudo "$dnf" group install development-tools -y
        else
            $dnf groupinstall "Development Tools" -y
        fi


        if [[ "$ID" == "rocky" && $(rpm -E %rhel) -ge 9 ]]; then
            $dnf --enablerepo=crb --allowerasing install "${packages[@]}" -y
        else
            $dnf install "${packages[@]}" g++ -y
        fi
        ;;
    "centos")
        yum install epel-release -y
        yum update -y
        yum groupinstall "Development Tools" -y

        yum install freetype-devel expat-devel fontconfig-devel cmake3 -y
        ln -s /usr/bin/cmake3 /usr/bin/cmake
        ;;
    "arch")
        # TODO: Add Arch Linux packages
        ;;
    *)
        echo -e "Unknown distribution, please manually find and install the relevant packages for your distro.\n"
        ;;
esac

if [[ -z $(which rustup 2>/dev/null) ]]; then
    echo "Installing rustup..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- --default-toolchain nightly -y
    source $HOME/.cargo/env
fi

echo -e "\nIf you're inside a toolbox or distrobox, you need to install the necessary display drivers for your GPU *inside* the container otherwise you will run into OpenGL issues."