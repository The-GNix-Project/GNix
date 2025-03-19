#!/usr/bin/env bash

path=$1

mkdir -p "$path"

cd $path

echo creating nixos config at: "$PWD"

# Enable flakes
export NIX_CONFIG="experimental-features = nix-command flakes"

HOSTNAME=$(hostname)

if [[ -f "/etc/nixos/configuration.nix" ]]; then
    echo "copying /etc/nixos/configuration.nix"
    cp "/etc/nixos/configuration.nix" ./configuration.nix
else 
    echo "nixos config path? "
    read config
    if [[$config == ""]]; then
        touch "configuration.nix"
    else
        cp $config .
    fi
fi
if [[ -f "/etc/nixos/hardware-configuration.nix" ]]; then
    echo "copying /etc/nixos/.nix"
    cp "/etc/nixos/hardware-configuration.nix" ./hardware-configuration.nix
else
    echo "hardware config path? "
    read hconfig
    if [[$hconfig == ""]]; then
        touch "hardware-configuration.nix"
    else
        cp $hconfig .
    fi
fi

# If no flake already then create a boiler plate flake
if [ ! -f flake.nix ]; then
    cat > flake.nix <<EOF
{
  description = "NixOS configuration";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs, ... }: {
    nixosConfigurations."$HOSTNAME" = nixpkgs.lib.nixosSystem {
      system = "$(uname -m)-linux";
      modules = [
        './configuration.nix' )
        './hardware-configuration.nix' )
      ];
    };
  };
}
EOF
    echo "Created new flake.nix for host: $HOSTNAME"
else
    echo "flake.nix already exists - skipping creation"
fi

# git init

# git add .

