# from nix_parser import parse
from typing import List
from subprocess import run, CalledProcessError
import os
import yaml

from nix_parser import parse_nix, find_key_pair

def nixos_config_init(path: str):
    BASH_PATH = "src/nix_manager/bash/nix_config_init.sh"
    try:
        run([BASH_PATH, path])
    except CalledProcessError as e:
        print("Script failed with error ", e)

FOLDER_TEMPLATES_PATH = "src/nix_manager/nixos_folder_templates"
TEMPLATES_PATH = "templates"

class nixFile(dict):
    def __init__(self, script, locate_modules=False):
        self.script = script
        self.parsed = parse_nix(script)

class nixosConfigDirectory:
    flakes: bool = False
    home_manager:bool = False
    modularise: bool = False
    path: str = ""
    
    # hashmap of folder_template
    folder_tree: dict = {}
    existing_config_files: list = []
    
    def __init__(self):
        with open(FOLDER_TEMPLATES_PATH + "/default.yaml") as f:
            self.folder_tree = yaml.load(f, Loader=yaml.Loader)   

        if os.path.isfile("/etc/nixos/hardware-configuration.nix"):
            self.existing_config_files.append("/etc/nixos/hardware-configuration.nix")
            with open("/etc/nixos/hardware-configuration.nix") as f:
                pass

        if os.path.isfile("/etc/nixos/configuration.nix"):
            self.existing_config_files.append("/etc/nixos/configuration.nix")
            with open("/etc/nixos/configuration.nix") as f:
                pass
                # existing_config = parse(f.read())
                # check for modules
                # parse modules, recurse all module dependencies
    
    def folder_structure(self, name):
        if os.path.isfile(FOLDER_TEMPLATES_PATH + f"/{name}.yaml"):
            with open(FOLDER_TEMPLATES_PATH + f"/{name}.yaml") as f:
                self.folder_tree = yaml.load(f, Loader=yaml.Loader)

    def add_file(self, path, locate_modules=True):
        if os.path.isfile(path):
            script = nixFile(script, locate_modules=True)
             
        
        

            
            