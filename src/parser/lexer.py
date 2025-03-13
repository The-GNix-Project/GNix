""" 
Script that contains all logic to take a Nix script and turn it into a stream of tokens

string      KEWORD_REGEX        Regex to match for a Nix keyword
string      IDENTIFIER_REGEX    Regex to match for a Nix identifier
class       _TokenTypes         Private class used for token type annotation
_TokenTypes tokenTypes          Variable used to perform logic etc with a token type. E.g. if token.type == tokenTypes.string:
list        TOKEN_MAPS          List of regex to match all different types of tokens
function    lex                 Tokenizes a nix script
"""

import re 

# from grammar import Token, TokenStream, NIX_KEYWORDS
from .grammar import Token, TokenStream, NIX_KEYWORDS

KEYWORD_REGEX = rf'\b({'|'.join(NIX_KEYWORDS)})\b'
IDENTIFIER_REGEX = rf"\b(?!(?:{'|'.join(NIX_KEYWORDS)})\b)[A-Za-z_][A-Za-z0-9_'-]*\b"

TOKENS = ['INTEGER','STRING','BOOL','NULL','FLOAT','PATH','KEYWORD','IDENTIFIER','LBRACE','RBRACE','LPAREN','RPAREN','LSPAREN','RSPAREN','COMMA','EQ','COLON','SEMICOLON','ELLIPSE','WHITESPACE','COMMENT','ATTR_SELECT','HAS_ATTR','LIST_CONCAT','MUL','DIV','SUB','ADD','LOGICAL_NOT','UPDATE','LT','LTE','GT','GTE','EQ_OP','NEQ_OP','LOGICAL_AND','LOGICAL_OR','IMPLIES','PIPE_LEFT','PIPE_RIGHT']

class _TokenTypes:
    """Private class used for token type annotation"""
    integer = "INTEGER"
    string = "STRING"
    bool = "BOOL"
    null = "NULL"
    float = "FLOAT"
    path = "PATH"
    keyword = "KEYWORD"
    identifier = "IDENTIFIER"
    left_brace, right_brace = "LBRACE", "RBRACE"
    left_paren, right_paren = "LPAREN", "RPAREN"
    left_square_paren, right_square_paren = "LSPAREN", "RSPAREN"
    comma = "COMMA"
    equals = "EQ"
    colon = "COLON"
    semicolon = "SEMICOLON"
    ellipse = "ELLIPSE"
    whitespace = "WHITESPACE"
    comment = "COMMENT"
    attribute_select = "ATTR_SELECT"
    has_attribute = "HAS_ATTR"
    list_concatenation = "LIST_CONCAT"
    multiply = "MUL"
    divide = "DIV"
    subtract = "SUB"
    add = "ADD"
    logical_not = "LOGICAL_NOT"
    update = "UPDATE"
    less_than = "LT"
    less_than_equals = "LTE"
    greater_than = "GT"
    greater_than_equals = "GTE"
    logical_equals = "EQ_OP"
    not_equal = "NEQ_OP"
    logical_and = "LOGICAL_AND"
    logical_or = "LOGICAL_OR"
    implies = "IMPLIES"
    left_pipe = "PIPE_LEFT"
    right_pipe = "PIPE_RIGHT"
    stream = "STREAM"

tokenTypes = _TokenTypes()

TOKEN_MAPS = [
    ('FLOAT',       r'-?\b\d+\.\d+([eE][+-]?\d+)?\b|-?\b\d+[eE][+-]?\d+\b'),# floating point number
    ('INTEGER',     r'-?\b\d+\b'),                                      # Integer
    ('STRING',      r'(?:"(?:\\.|[^"\\])*")|(?:\'(?:\\.|[^\'\\])*\')'), # String literal with escapes
    ('BOOL',        r'\b(true|false)\b'),                               # Boolean 
    ('NULL',        r'\bnull\b'),                                       # NULL
    ('PATH',        r'(\./|/)[a-zA-Z0-9_\-][a-zA-Z0-9_\-./]*'),         # Nix Path datatype
    ('KEYWORD',     KEYWORD_REGEX),                                     # Keyword
    ('IDENTIFIER',  IDENTIFIER_REGEX),                                  # Identifiers
    ('LBRACE',      r'\{'),                                             # Left brace
    ('RBRACE',      r'\}'),                                             # Right brace
    ('LPAREN',      r'\('),                                             # Left parenthesis
    ('RPAREN',      r'\)'),                                             # Right parenthesis
    ('LSPAREN',     r'\['),                                             # Left parenthesis
    ('RSPAREN',     r'\]'),                                             # Right parenthesis
    ('COMMA',       r'\,'),                                             # Comma
    ('COLON',       r':'),                                              # Colon
    ('SEMICOLON',   r';'),                                              # Semicolon
    ('ELLIPSE',      r'\.\.\.'),                                        # ELlipse
    ('WHITESPACE',  r'\s+'),                                            # Whitespace (ignored)
    ('COMMENT',     r'#.*'),                                            # Comment (ignored)

    # Operators
    ('ATTR_SELECT',   r'(?<!\d)\s*\.\s*(?!\d)'),        # Attribute selection (attrset . attrpath)
    ('IMPLIES',       r'->'),                           # Logical implication (bool -> bool)
    ('HAS_ATTR',      r'\?'),                           # Has attribute (attrset ? attrpath)
    ('LIST_CONCAT',   r'\+\+'),                         # List concatenation (list ++ list)
    ('UPDATE',        r'\/\/'),                         # Attribute set update (attrset // attrset)
    ('PIPE_LEFT',     r'\|>'),                          # Pipe operator (expr |> func)
    ('PIPE_RIGHT',    r'<\|'),                          # Pipe operator (func <| expr)
    ('MUL',           r'\*'),                           # Multiplication (number * number)
    ('DIV',           r'\/'),                           # Division (number / number)
    ('SUB',           r'-'),                            # Subtraction (number - number)
    ('ADD',           r'\+'),                           # Addition (number + number)
    ('LTE',           r'<='),                           # Less than or equal to (expr <= expr)
    ('GTE',           r'>='),                           # Greater than or equal to (expr >= expr)
    ('GT',            r'>'),                            # Greater than (expr > expr)
    ('EQ_OP',         r'=='),                           # Equality (expr == expr)
    ('LT',            r'<'),                            # Less than (expr < expr)
    ('NEQ_OP',        r'!='),                           # Inequality (expr != expr)
    ('LOGICAL_AND',   r'&&'),                           # Logical conjunction (AND) (bool && bool)
    ('LOGICAL_NOT',   r'!'),                            # Logical negation (NOT) (! bool)
    ('LOGICAL_OR',    r'\|\|'),                         # Logical disjunction (OR) (bool || bool)
    ('EQ',            r'='),                            # Equals sign
]

def lex(code: str) -> TokenStream:
    """Tokenizes a Nix script

    :param code: Nix script to lex
    :type code: str
    :return: returns a stream of tokens
    :rtype: TokenStream
    """
    tok_regex = '|'.join(f'(?P<{name}>{pattern})' for name, pattern in TOKEN_MAPS)
    tokens = TokenStream()
    
    for mo in re.finditer(tok_regex, code):
        kind = mo.lastgroup
        value = mo.group()
        if kind == tokenTypes.whitespace or kind == tokenTypes.comment:
            continue
        tokens.append(Token(kind, value))
        print(tokens[-1])
    
    return tokens
    