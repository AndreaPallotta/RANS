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

install_npm() {
    local package="$1"

    if [[ -z "$package" ]]; then
        echo "Package not specified. Skipping..."
        return 1
    fi

    if ! npm list -g "$package" &>/dev/null; then
        echo "NPM $package is not installed. Installing..."
        sudo npm install -g "$package"
        echo "NPM $package successfully installed!"
    else
        echo "NPM $package is already installed. Skipping..."
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

    if [[ -d "$src" ]]; then
        sudo cp -r "$src" "$dest"
    elif [[ -f "$src" ]]; then
        sudo cp "$src" "$dest"
    else
        echo "$src does not exist."
        return 1
    fi

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
    local dest_dir="$2"
    local dest="$dest_dir/$(basename $src)"

    if [[ ! -e "$src" ]]; then
        echo "$src not found"
        return 1
    fi

    if [[ ! -d "$dest_dir" ]]; then
        echo "$dest_dir not found"
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

install_cargo() {
    /tmp/sh.rustup.rs -y
    source ~/.bashrc
    source $HOME/.cargo/env
}

cleanup() {
    local exit_status=$?
    if [ $exit_status -ne 0 ]; then
        echo "An error occurred while building the client app. Exit status: $exit_status"
        sudo dnf remove nginx arangodb3 -y
        sudo rm -rf "$rans_remote"
        sudo rm -rf "$client_remote"
        sudo rm -rf "$log_remote"
        sudo rm -rf "$systemd_remote"/rans.service.d
        sudo rm -rf "$systemd_remote"/rans.api.service.d
        sudo rm -rf "$bin_remote"/server
    fi
}

# Variables
client_dist="./client/dist"
configs_local="./config"
api_bin="./server/target/release/server"
ansible_playbook="./playbook.yaml"
services="./rans.service.d"

client_remote="/var/www/rans/public"
rans_remote="/etc/rans"
bin_remote="/usr/bin"
systemd_remote="/etc/systemd/system"
nginx="/etc/nginx"
log_remote="/var/log/rans"
net_remote="/etc/sysconfig/network-scripts"

# Start of script

trap cleanup EXIT

echo
echo "============ Install Dependencies ============"
echo

sudo dnf module install nodejs:18 -y
check_deps "ansible-core"
check_deps "nodejs"
setup_python "3.9"
check_deps "ansible-core"
pip3 install pexpect
ansible-galaxy collection install ansible.posix
install_npm "svelte"

echo
echo "============ Create Remote Paths ============"
echo

create_path "$rans_remote" true
create_path "$client_remote" true
create_path "$log_remote" true

sudo chown -R $USER:$USER "$log_remote"
sudo chmod u+w "$log_remote"
sudo chmod a+x ./db_setup.sh

echo
echo "============ Run Ansible Playbook & Set Up Database ============"
echo

ansible-playbook "$ansible_playbook" --ask-become-pass
if [ $? -ne 0 ]; then
  echo "The Ansible playbook failed. Stopping gracefully..."
  exit 1
fi

if ! rpm -q "cargo" &> /dev/null; then
    echo "Installing cargo..."
    install_cargo
    echo "Successfully installed cargo!"
fi

sudo chmod a+x ./db_setup.sh && ./db_setup.js

echo
echo "============ Set Up Files ============"
echo

if [[ ! -d "$client_dist" ]]; then
    echo "Building client app..."
    (cd client && npm install && npm run build)
    if [ $? -ne 0 ]; then
        echo "Error: Failed to build client app"
        exit 1
    else
        echo "Client app successfully built!"
        echo
    fi
fi

if [[ ! -f "$api_bin" ]]; then
    echo "Building API binary..."
    (cd server && cargo build --release --bin server)
    if [ $? -ne 0 ]; then
        echo "Error: Failed to build API binary. Stopping gracefully..."
        exit 1
    else
        echo "API binary successfully built!"
        echo
    fi
fi

copy ./rans.service "$systemd_remote"
copy "$services" "$systemd_remote"
copy "$configs_local"/nginx.conf "$nginx"
copy "$configs_local"/config.toml "$rans_remote"
copy "$configs_local"/ifcfg-ens192 "$net_remote"
copy "$client_dist"/. "$client_remote"
copy "$api_bin" "$bin_remote"

sudo chown root:systemd-journal "$systemd_remote"/rans.service.d/rans.api.service
sudo chown root:systemd-journal "$systemd_remote"/rans.service
sudo chown root:systemd-journal "$bin_remote"/server
sudo chmod 644 "$systemd_remote"/rans.service.d/rans.api.service
sudo chmod 644 "$systemd_remote"/rans.service
sudo chmod 744 "$bin_remote"/server
sudo semanage fcontext -a -t bin_t "$bin_remote"/server
sudo restorecon -v "$bin_remote"/server
sudo chcon -Rt httpd_sys_content_t /var/www/
sudo setsebool httpd_can_network_connect on -P

echo

create_symlink "$systemd_remote/rans.service.d/rans.api.service" "$systemd_remote"

echo



sudo systemctl daemon-reload
sudo systemctl restart NetworkManager
sudo systemctl start nginx.service arangodb.service rans.api.service

echo
echo "============ Set Up Cron Job ============"
echo

create_zip_cron

echo
echo "Successfully executed script!"
echo

# End of Script