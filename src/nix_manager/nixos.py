# SPDX-License-Identifier: GPL-3.0-or-later
#
# This file is part of GNix.
#
# GNix is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or any later version.
#
# GNix is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with GNix.  If not, see <https://www.gnu.org/licenses/>.

# from nix_parser import parse
from typing import List
from subprocess import run, CalledProcessError
import os
import yaml

from nix_parser import parse_nix, find_key_pair

{
    "configurationName": "name",
    "configurationLocation": "location",
    "git": True,
    "github": True,
    "gitlab": True,
    "flakes": True,
    "homeManager": True,
    "existingConfig": "location",
    "existingHConfig": "location",
    "directoryTree": [
        {"hosts": {
            "host1": ["hardware-configuration.nix", "configuration.nix"]
        }},
        {"users": {
            "user1": []
        }},
        {"modules": {
            "module1": "default.nix"
        }},
        "flake.nix"
    ]
}

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
    
    def folder_structure(self, name):
        if os.path.isfile(FOLDER_TEMPLATES_PATH + f"/{name}.yaml"):
            with open(FOLDER_TEMPLATES_PATH + f"/{name}.yaml") as f:
                self.folder_tree = yaml.load(f, Loader=yaml.Loader)

    def add_file(self, path, locate_modules=True):
        if os.path.isfile(path):
            script = nixFile(script, locate_modules=True)
             
        
        

            
            