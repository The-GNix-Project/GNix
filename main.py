from nix_parser import parse_nix, find_key_pair

parsed = parse_nix("{imports = {a=1; b=2; c=3;};}")
print(find_key_pair(parsed, "imports").get("Map").get("bindings"))
