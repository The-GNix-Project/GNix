from nix_parser import parse_nix

print(type(parse_nix("{a, b, c}: {g = with pkgs; [a b c];}")))