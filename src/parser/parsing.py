""" 
Contains everything to turn a TokenStream into an Abstract Syntax Tree

dict        _BRACKET_MAP        used to define pairs of brackets
function    _parse_block        takes a TokenStream and index of first bracket then recursively splits it into more TokenStreams, grouping them by brackets {} [] and ()
function    parse_brackets      takes a raw TokenStream then recursively splits it into more TokenStreams, grouping them by brackets {} [] and ()
function    is_nix_array        checks if a given TokenStream is a Nix Array
function    parse_arguments     Parses arguments of a Nix function
function    build_assignment    builds/parses Nix assignment (var = val;)
function    array               parses a Nix array datatype
function    _parse              Idk what it does yet lol, very in development
"""

from json import dump
from os import remove, path
from typing import Tuple

from .lexer import lex, tokenTypes
from .grammar import Token, TokenStream, PRIMITIVE_TYPES
    

# Mapping opening tokens to their expected closing token.
_BRACKET_MAP = {
    tokenTypes.left_brace:          tokenTypes.right_brace,
    tokenTypes.left_paren:          tokenTypes.right_paren,
    tokenTypes.left_square_paren:   tokenTypes.right_square_paren,
}

def _parse_block(tokens: TokenStream, i) -> TokenStream:
    """takes a TokenStream and index of first bracket then recursively splits it into more TokenStreams, grouping them by brackets {} [] and ()

    :param tokens: TokenStream to group
    :type tokens: TokenStream
    :param i: index of opening bracket
    :type i: int
    :raises SyntaxError: raises error if index provided has no valid opening bracket
    :raises SyntaxError: raises error if no closing bracket exists
    :return: returns the grouped TokenStream
    :rtype: TokenStream
    """
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

def parse_brackets(tokens: TokenStream) -> TokenStream:
    """takes a raw TokenStream then recursively splits it into more TokenStreams, grouping them by brackets {} [] and ()

    :param tokens: Raw TokenStream to parse
    :type tokens: TokenStream
    :return: TokenStream grouped by brackets
    :rtype: TokenStream
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
    """checks if a given TokenStream is a Nix Array

    :param token_list: TokenStream to check
    :type token_list: TokenStream
    :return: True or False
    :rtype: bool
    """
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

def parse_arguments(stream:TokenStream) -> list:
    """Parses arguments of a function

    :param stream: stream to parse
    :type stream: TokenStream
    :return: list of arguments
    :rtype: list
    """
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
    """builds/parses Nix assignment (var = val;)

    :param i: index of equals sign
    :type i: int
    :param token_stream: current token stream
    :type token_stream: TokenStream
    :param token: current token (should be an equals sign)
    :type token: Token
    :param results: current list of parsing results
    :type results: list
    :return: returns the index, current token and results
    :rtype: Tuple[int, Token, list]
    """
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

def array(token_stream: TokenStream | list) -> list:
    """parses a Nix array datatype

    :param token_stream: Nix array to build syntax tree from
    :type token_stream: TokenStream | list
    :return: returns a list containing elements in the array    
    :rtype: list
    """
    print(token_stream)
    if isinstance(token_stream, list):
        return token_stream
    if token_stream[0].type == "LSPAREN":
        token_stream.pop(0)
    if token_stream[-1].type == "RSPAREN":
        token_stream.pop(-1)
    array = []
    for token in token_stream:
        array.append({token.type: token.content})
        
    return array



# Recursive parser
def _parse(token_stream: TokenStream) -> dict:
    """Idk what it does yet lol, very in development"""
    # Check if current level is an array FIRST
    if is_nix_array(token_stream):
        return {"ARRAY": array(_parse(token_stream[1:-1]))}
                
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