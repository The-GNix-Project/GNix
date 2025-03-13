{ config, pkgs, lib, inputs, instring ? "myVal", ... }:
{
experimental-features = [ "nix-command" "flakes" ["test" 2] {a=2;b=3}];

imports =
    [ # Include the results of the hardware scan.
    ./hardware-configuration.nix
    # Computer specific settings
    # ./mavic.nix
    ];
    
myVar = 4;
}