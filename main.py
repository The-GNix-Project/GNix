import os, json

from src import lex
from src.parser.parsing import parse_brackets, _parse

script = """ 
{ config, pkgs, lib, inputs, instring ? "myVal", ... }:
{
experimental-features = [ "nix-command" "flakes" ["test" 2] {a=2;b=3}];

imports =
    [ # Include the results of the hardware scan.
    ./hardware-configuration.nix
    # Computer specific settings
    # ./mavic.nix
    ];
    
myVar = 4;
}
"""
brackets = parse_brackets(lex(script))
parsed = _parse(brackets)
if os.path.isfile("AST.json"): os.remove("AST.json")
with open("AST.json", "x") as file:
    json.dump(parsed, file)
