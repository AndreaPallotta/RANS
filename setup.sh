#!/usr/bin/env bash

# Functions
check_deps() {
    local package="$1"

    if ! rpm -q "$package" &> /dev/null; then
        echo "$package is not installed. Installing..."
        sudo dnf install -y "$package"
        echo "$package successfully installed!"
    else
        echo "$package is already installed. Skipping..."
    fi
}

setup_python() {
    version="$1"
    only_python3="$2"

    if [[ -z "$version" ]]; then
        version="3.9"
    fi

    check_deps "python${version/./}"

    if [[ "$only_python3" = "true" ]]; then
        sudo alternatives --set python /usr/bin/python3
    fi
    sudo alternatives --set python3 /usr/bin/python3
}

create_path() {
    local path="$1"
    local is_sudo="$2"

    if [[ -d "$path" ]]; then
        echo "$path already exists. Skipping..."
        return 0
    fi

    if [[ "$is_sudo" == true ]]; then
        sudo mkdir -p "$path"
    else
        mkdir -p "$path"
    fi
    echo "$path successfully created!"
}

copy() {
    local src="$1"
    local dest="$2"

    if [[ ! -e "$src" ]]; then
        echo "$src does not exist."
        return 1
    fi

    sudo cp -r "$src" "$dest"
    if [[ $? -ne 0 ]]; then
        echo "Copy failed"
        return 1
    fi
    echo "$src successfully copied in $dest"
}

clear_directory() {
    local path="$1"

    if [[ ! -d "$path" ]]; then
        echo "Directory clear failed: path is not a folder"
        sudo rm -rf "$path"/*
    fi
}

create_symlink() {
    local src="$1"
    local dest="$2"

    if [[ ! -e "$src" ]]; then
        echo "$src not found"
        return 1
    fi

    if [[ ! -e "$dest" ]]; then
        echo "$dest not found"
        return 1
    fi

    if [[ -L "$dest" ]]; then
        echo "$dest is already a symbolic link"
        return 1
    fi

    sudo ln -s "$src" "$dest"
    if [[ $? -ne 0 ]]; then
        echo "Symlink failed"
        return 1
    fi

    echo "Successfully symlinked $src to $dest"
}

create_zip_cron() {
    local cron_job="55 11 * * * if [ $(du -sm /var/log/rans | awk '{print $1}') -gt 50 ]; then cd /var/log/rans && zip -r log_files.zip . && rm -r *; fi"

    if crontab -l | grep -q "$cron_job"; then
        echo "Cron job already exists. Skipping..."
    else
        (crontab -l 2>/dev/null; echo "$cron_job") | crontab -
        echo "Successfully added cron job"
    fi
}

source_cargo() {
    source ~/.bashrc
    source $HOME/.cargo/env
}

# Variables
client_dist="./client/dist"
nginx_configs="./config"
server_bin="./server/target/release/server"
ansible_playbook="./playbook.yaml"
services="./rans.service.d"

client_remote="/var/www/rans/public"
rans_remote="/etc/rans"
bin_remote="/usr/bin"
systemd_remote="/etc/systemd/"
nginx_availables="/etc/nginx/sites-available"
nginx_enabled="/etc/nginx/sites-enabled"
log_remote="/var/log/rans"

# Start of script

echo
echo "============ Install Dependencies ============"
echo

check_deps "ansible-core"
sudo dnf module install nodejs:18 -y
check_deps "nodejs"
setup_python "3.9"
check_deps "ansible-core"
pip3 install pexpect
ansible-galaxy collection install ansible.posix
sudo npm install -g vite

echo
echo "============ Create Remote Paths ============"
echo

create_path "$rans_remote" true
create_path "$client_remote" true
create_path "$systemd_remote" true
create_path "$log_remote" true

echo
echo "============ Run Ansible Playbook ============"
echo

ansible-playbook "$ansible_playbook" --ask-become-pass
source_cargo

echo
echo "============ Set Up Files ============"
echo

if [[ ! -e "$client_dist" ]]; then
    echo "Building client app..."
    (cd client && npm install && npm run build)
    echo "Client app successfully built!"
    echo
fi

if [[ ! -e "$server_bin" ]]; then
    echo "Building server binary..."
    (cd server && cargo build --release --bin server)
    echo "Server binary successfully built!"
    echo
fi

copy "$services" "$systemd_remote"
copy "$nginx_configs/nginx.conf" "$rans_remote"
copy "$nginx_configs/config.toml" "$rans_remote"
copy "$nginx_configs/rans.iste444.com" "$nginx_availables"
copy "$nginx_configs/ransapi.iste444.com" "$nginx_availables"
copy "$client_dist"/. "$client_remote"
copy "$server_bin" "$bin_remote"

echo

create_symlink "$nginx_availables/rans.iste444.com" "$nginx_enabled/rans.iste444.com"
create_symlink "$nginx_availables/ransapi.iste444.com" "$nginx_enabled/ransapi.iste444.com"

echo

sudo systemctl daemon-reload
sudo systemctl restart nginx &>/dev/null

echo
echo "============ Set Up Cron Job ============"
echo

create_zip_cron

echo
echo "Successfully executed script!"
echo

# End of Script