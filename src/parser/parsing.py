from lexer import lex
from grammar import Array, Token, TokenStream

# from .lexer import lex
# from .grammar import Array, Token, TokenStream
    

# Mapping opening tokens to their expected closing token.
_BRACKET_MAP = {
    "LBRACE": "RBRACE",
    "LPAREN": "RPAREN",
    "LSPAREN": "RSPAREN",
}

def _parse_block(tokens: TokenStream, i):
    token = tokens[i]
    if token.type not in _BRACKET_MAP:
        raise SyntaxError(f"Expected opening token at index {i}, got {token}")
    open_token = token
    closing_expected = _BRACKET_MAP[open_token.type]
    content = TokenStream()  # will hold inner tokens or nested blocks
    i += 1  # move past the opening token

    while i < len(tokens):
        token = tokens[i]
        if token.type in _BRACKET_MAP:
            # Found a nested block – parse it recursively.
            block, i = _parse_block(tokens, i)
            content.append(block)
        elif token.type == closing_expected:
            # Found the matching closing token.
            close_token = token
            i += 1
            return TokenStream([open_token] + content.stream + [close_token]), i
        else:
            # Regular token; append it.
            content.append(token)
            i += 1

    raise SyntaxError(f"Missing closing token for {open_token}")

def _parse_brackets(tokens: TokenStream):
    """
    Parse an entire token stream into a list of blocks.
    Tokens outside of any block will be added as individual items.
    """
    result = TokenStream()
    i = 0
    while i < len(tokens):
        if tokens[i].type in _BRACKET_MAP:
            block, i = _parse_block(tokens, i)
            result.append(block)
        else:
            result.append(tokens[i])
            i += 1
    return result

# Array detection
def is_nix_array(token_list: TokenStream) -> bool:
    if not token_list:
        return False
    
    try:
        first = token_list[0]
        last = token_list[-1]
        
        # Check if first/last elements are actually Tokens
        if not (isinstance(first, Token) and isinstance(last, Token)):
            return False
            
        return first.type == "LSPAREN" and last.type == "RSPAREN"
        
    except IndexError:
        return False

# Recursive parser
def _parse(token_list: TokenStream) -> dict:
    # Check if current level is an array FIRST
    if is_nix_array(token_list):
        return {"ARRAY": Array(token_list)}
        
    # Then process nested streams
    results = []
    for token in token_list:
        if isinstance(token, TokenStream):
            print("Recursing into nested stream")
            results.append(_parse(token))
    
    return {"BLOCK": results}
    
if __name__ == "__main__":
    script = """ 
    { config, pkgs, lib, inputs, ... }:

    {
    nix.settings.experimental-features = [ "nix-command" "flakes" ];

    imports =
        [ # Include the results of the hardware scan.
        ./hardware-configuration.nix
        # Computer specific settings
        # ./mavic.nix
        ];
        
    attrs = {x=1;b=3;c={n=2}d=x:x+1};
    }

    """
    brackets = _parse_brackets(lex(script))
    print(_parse(brackets))
