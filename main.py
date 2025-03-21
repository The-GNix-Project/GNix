from nix_parser import parse_nix, find_key_pair

parsed = parse_nix("{imports = {1, 2, 3}}")

print(find_key_pair(parsed, "imports"))
