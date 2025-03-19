from src.nix_manager.nixos import nixosConfigDirectory
from nix_parser import nix

config = nixosConfigDirectory()
print(config.folder_tree)
config.folder_structure("default-with-users")
print(config.folder_tree)
