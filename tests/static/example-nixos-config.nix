{ config, pkgs, lib, inputs, ... }:

{
  nix.settings.experimental-features = [ "nix-command" "flakes" ];

  imports =
    [
      ./hardware-configuration.nix
    ];

  # Bootloader.
  boot.loader.systemd-boot.enable = true;
  boot.loader.efi.canTouchEfiVariables = true;

  networking.hostName = "nix"; # Define your hostname.

  # Enable networking
  networking.networkmanager.enable = true;

  # Define a user account. Don't forget to set a password with ‘passwd’.
  users.users.user = {
    isNormalUser = true;
    description = "A User";
    extraGroups = [ "networkmanager" "wheel" ];
    packages = with pkgs; [
      home-manager
    ];
  };

  # Allow unfree packages
  nixpkgs.config.allowUnfree = true;
  nixpkgs.config.pulseaudio = true;

  # List packages installed in system profile. To search, run:
  # $ nix search wget
  environment.systemPackages = with pkgs; [
    git vim findutils wget
    findutils.locate
    clinfo
    system-config-printer avahi
    gnumake
  ];

  fonts.packages = [ ] ++ builtins.filter lib.attrsets.isDerivation (builtins.attrValues pkgs.nerd-fonts);

  programs = {
    thunar.enable = true;
    dconf.enable = true;
    sway.enable = true;
    kdeconnect.enable = true;
    partition-manager.enable = true;
  };
}         
        