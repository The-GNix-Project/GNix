from subprocess import run, CalledProcessError

from nix_parser import parse_nix

def nixos_config_init(path: str):
    BASH_PATH = "/home/archie/Documents/GNix/src/nixtools/bash/nix_config_init.sh"
    try:
        run([BASH_PATH, path])
    except CalledProcessError as e:
        print("Script failed with error ", e)