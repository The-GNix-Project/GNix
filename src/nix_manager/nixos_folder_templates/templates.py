import yaml
import os

AVAILABLE_TEMPLATES = ["default", "default_with_users"]

def build_tree(structure):
    """Recursively builds a dictionary-based tree from the parsed YAML structure."""
    if not isinstance(structure, dict):
        return None  # Prevents incorrect structure parsing

    tree = {}

    for key, value in structure.items():
        if isinstance(value, dict):  # Nested directories
            tree[key] = build_tree(value) or {}  # Ensure empty dicts for empty folders
        else:
            tree[key] = None  # Placeholder for files

    return tree

def parse_yaml_tree(yaml_path):
    """Parses the YAML file and returns the tree as a dictionary."""
    with open(yaml_path, "r") as file:
        data = yaml.safe_load(file)

    return {"root": build_tree(data["root"])} if "root" in data else {}

TEMPLATES = {}
directory = "src/nix_manager/nixos_folder_templates"
yaml_files = [directory + "/" + f for f in os.listdir(directory) if f.endswith(".yaml")]
import json
for file in yaml_files:
    print(json.dumps(parse_yaml_tree(file), indent=2))
