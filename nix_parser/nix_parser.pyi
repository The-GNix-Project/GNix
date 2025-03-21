def parse_nix(nix_script: str) -> dict:
    """Parses a Nix script into a dictionary
     # Arguments
     `nix_script` - A string containing the Nix script to parse.

     # Returns
     A dictionary representing the parsed Nix script.

     # Errors
     Returns a `PyRuntimeError` if parsing or serialization fails.
     ```
     """

def find_key_pair(node: dict|list, key: str) -> dict|None:
    """
    Recursively search the AST for a KeyValue node where `from` is `key`
    and return the first instance of `key` being defined
    # Arguments
    `node` - A dictionary, a nix AST
    `key`  - key to search for

    # Returns
    `dict|None` - the value defined in `key`
    """