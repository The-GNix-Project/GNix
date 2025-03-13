import re 

# from grammar import Token, TokenStream, NIX_KEYWORDS, OPERATORS
from .grammar import Token, TokenStream, NIX_KEYWORDS, OPERATORS

KEYWORD_REGEX = rf'\b({'|'.join(NIX_KEYWORDS)})\b'
IDENTIFIER_REGEX = rf"\b(?!(?:{'|'.join(NIX_KEYWORDS)})\b)[A-Za-z_][A-Za-z0-9_'-]*\b"

TOKENS = ['INTEGER','STRING','BOOL','NULL','FLOAT','PATH','KEYWORD','IDENTIFIER','LBRACE','RBRACE','LPAREN','RPAREN','LSPAREN','RSPAREN','COMMA','EQ','COLON','SEMICOLON','ELLIPSE','WHITESPACE','COMMENT','ATTR_SELECT','HAS_ATTR','LIST_CONCAT','MUL','DIV','SUB','ADD','LOGICAL_NOT','UPDATE','LT','LTE','GT','GTE','EQ_OP','NEQ_OP','LOGICAL_AND','LOGICAL_OR','IMPLIES','PIPE_LEFT','PIPE_RIGHT']

class _TokenTypes:
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
    ('INTEGER',     r'-?\b\d+\b'),                          # Integer
    ('STRING',      r'"(?:\\.|[^"\\])*"'),                  # String literal with escapes
    ('BOOL',        r'\b(true|false)\b'),                   # Boolean 
    ('NULL',        r'\bnull\b'),                           # NULL
    ('FLOAT',       r'-?\b\d+(\.\d+)?([eE][+-]?\d+)?\b'),   # floating point number
    ('PATH',        r'(\./|/)[a-zA-Z0-9_\-./]+'),           # Nix Path datatype
    ('KEYWORD',     KEYWORD_REGEX),                         # Keyword
    ('IDENTIFIER',  IDENTIFIER_REGEX),                      # Identifiers
    ('LBRACE',      r'\{'),                                 # Left brace
    ('RBRACE',      r'\}'),                                 # Right brace
    ('LPAREN',      r'\('),                                 # Left parenthesis
    ('RPAREN',      r'\)'),                                 # Right parenthesis
    ('LSPAREN',     r'\['),                                 # Left parenthesis
    ('RSPAREN',     r'\]'),                                 # Right parenthesis
    ('COMMA',       r'\,'),                                 # Comma
    ('EQ',          r'='),                                  # Equals sign
    ('COLON',       r':'),                                  # Colon
    ('SEMICOLON',   r';'),                                  # Semicolon
    ('ELLIPSE',      r'\.\.\.'),                            # ELlipse
    ('WHITESPACE',  r'\s+'),                                # Whitespace (ignored)
    ('COMMENT',     r'#.*'),                                # Comment (ignored)

    # Operators
    ('ATTR_SELECT',    r'\.'),              # Attribute selection (attrset . attrpath)
    ('HAS_ATTR',       r'\?'),              # Has attribute (attrset ? attrpath)
    ('LIST_CONCAT',    r'\+\+'),            # List concatenation (list ++ list)
    ('MUL',           r'\*'),               # Multiplication (number * number)
    ('DIV',           r'\/'),               # Division (number / number)
    ('SUB',           r'-'),                # Subtraction (number - number)
    ('ADD',           r'\+'),               # Addition (number + number)
    ('LOGICAL_NOT',   r'!'),                # Logical negation (NOT) (! bool)
    ('UPDATE',        r'\/\/'),             # Attribute set update (attrset // attrset)
    ('LT',            r'<'),                # Less than (expr < expr)
    ('LTE',           r'<='),               # Less than or equal to (expr <= expr)
    ('GT',            r'>'),                # Greater than (expr > expr)
    ('GTE',           r'>='),               # Greater than or equal to (expr >= expr)
    ('EQ_OP',         r'=='),               # Equality (expr == expr)
    ('NEQ_OP',        r'!='),               # Inequality (expr != expr)
    ('LOGICAL_AND',   r'&&'),               # Logical conjunction (AND) (bool && bool)
    ('LOGICAL_OR',    r'\|\|'),             # Logical disjunction (OR) (bool || bool)
    ('IMPLIES',       r'->'),               # Logical implication (bool -> bool)
    ('PIPE_LEFT',     r'\|>'),              # Pipe operator (expr |> func)
    ('PIPE_RIGHT',    r'<\|'),              # Pipe operator (func <| expr)
]

def lex(code):
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
    