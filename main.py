from nix_parser import parse_nix, find_key_pair
import json

parsed = parse_nix('"hello world"')
print(parsed)