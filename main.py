from src.nix_manager.nixos import parse_nix, find_key_pair

with open("test.nix") as f:
    parsed = parse_nix(f.read())

print(find_key_pair(parsed, "imports"))
