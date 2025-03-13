from lexer import lex, tokenTypes
from grammar import Token, TokenStream, PRIMITIVE_TYPES, array

from json import dump
from os import remove, path
from typing import Tuple

# from .lexer import lex
# from .grammar import Array, Token, TokenStream
    

# Mapping opening tokens to their expected closing token.
_BRACKET_MAP = {
    tokenTypes.left_brace:          tokenTypes.right_brace,
    tokenTypes.left_paren:          tokenTypes.right_paren,
    tokenTypes.left_square_paren:   tokenTypes.right_square_paren,
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
    
    if isinstance(token_list, Token):
        return False
    
    try:
        first = token_list[0]
        last = token_list[-1]
        
        # Check if first/last elements are actually Tokens
        if not (isinstance(first, Token) and isinstance(last, Token)):
            return False
            
        return first.type == tokenTypes.left_square_paren and last.type  == tokenTypes.right_square_paren
        
    except IndexError:
        return False

def is_attr_set(stream: TokenStream) -> bool: ...
def is_function(stream: TokenStream) -> bool: ...

def parse_arguments(stream:TokenStream) -> list:
    if stream[0].type == tokenTypes.left_brace: stream.pop(0)
    if stream[-1].type == tokenTypes.right_brace: stream.pop(-1)
    arguments = []
    for i, token in enumerate(stream):
        if token.type == tokenTypes.identifier:
            argument = {}
            try:
                if stream[i+1].type == tokenTypes.has_attribute:
                    argument["DEFAULT"] = _parse(TokenStream([stream[i+2]]))[0]
            except IndexError: pass
            argument["ARGUMENT"] = token.content
            arguments.append(argument)
    return arguments

def build_assignment(i: int, token_stream: TokenStream, token: Token, results: list)-> Tuple[int, Token, list]:
    if token.type == tokenTypes.equals: 
        try:
            left = token_stream[i-1]
            right = token_stream[i+1]
            if is_nix_array(right):
                right = Token("ARRAY", _parse(right))
            if (left.type == tokenTypes.identifier and
                token_stream[i+2].type == tokenTypes.semicolon and
                token_stream[i-2].type != tokenTypes.attribute_select):
                if right.type in PRIMITIVE_TYPES:
                    results.append({"ASSIGNMENT":{"VARIABLE":left.content, "VALUE":{right.type:right.content}}})
                    i += 2
                if right.type == "ARRAY":
                    results.append({"ASSIGNMENT":{"VARIABLE":left.content, "VALUE":{right.type:right.content["ARRAY"]}}})
                    i += 2
        except IndexError: pass
    return i, results  

# Recursive parser
def _parse(token_stream: TokenStream) -> dict:
    # Check if current level is an array FIRST
    if is_nix_array(token_stream):
        return {"ARRAY": array(_parse(token_stream[1:-1]))}
        
    if is_attr_set(token_stream): pass
    if is_function(token_stream): pass
        
    # Then process nested streams
    results = []
    i = 0
    while i < len(token_stream):
        token = token_stream[i]
        try:
            if (token_stream[i-1].type==tokenTypes.stream and
                token.type == tokenTypes.colon and
                token_stream[i+1].type==tokenTypes.stream):
                return {"FUNCTION":{"ARGUMENTS":parse_arguments(token_stream[i-1]), "CONTENTS":_parse(token_stream[i+1])}}
        except IndexError: pass
        
        if isinstance(token, TokenStream):
            results.append(_parse(token))
        if token.type == tokenTypes.equals: 
            try:
                left = token_stream[i-1]
                right = token_stream[i+1]
                if is_nix_array(right):
                    right = Token("ARRAY", _parse(right))
                if (left.type == tokenTypes.identifier and
                    token_stream[i+2].type == tokenTypes.semicolon and
                    token_stream[i-2].type != tokenTypes.attribute_select):
                    if right.type in PRIMITIVE_TYPES:
                        results.append({"ASSIGNMENT":{"VARIABLE":left.content, "VALUE":{right.type:right.content}}})
                        i += 2
                        try: token = token_stream[i]
                        except IndexError: break
                    if right.type == "ARRAY":
                        results.append({"ASSIGNMENT":{"VARIABLE":left.content, "VALUE":{right.type:right.content["ARRAY"]}}})
                        i += 2
                        try: token = token_stream[i]
                        except IndexError: break
            except IndexError: pass
        if token.type in PRIMITIVE_TYPES: 
            results.append({token.type:token.content})
        i += 1
    
    return results
    
if __name__ == "__main__":
    script = """ 
    { config ? "myVal", pkgs, lib, inputs, instring, ... }:
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

    { config ? "myVal", pkgs, lib, inputs, instring, ... }:
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
    brackets = _parse_brackets(lex(script))
    parsed = _parse(brackets)
    if path.isfile("AST.json"): remove("AST.json")
    with open("AST.json", "x") as file:
        dump(parsed, file)
