from nix_parser import parse

class nixosConfigDirectory:
  flakes = False
  home_manager = False
  modularise = False
  path = ""
  folder_tree = {"root": []}
  existing_config_files = []

  
