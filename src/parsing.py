from typing import override

import re
NIX_KEYWORDS = ["assert",
                "else",
                "if",
                "in",
                "inherit",
                "let",
                "or",
                "rec",
                "then",
                "with"
]
KEYWORD_REGEX = rf'\b({'|'.join(NIX_KEYWORDS)})\b'
IDENTIFIER_REGEX = rf"(?!{'|'.join(NIX_KEYWORDS)})" + r"[A-Za-z_][A-Za-z0-9_'-]*"
            
class Lexer:
    PRIMITIVE_TOKENS = [
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
        ('EQ',          r'='),                                  # Equals sign
        ('COLON',       r':'),                                  # Colon
        ('SEMICOLON',   r';'),                                  # Semicolon
        ('QMARK',       r'\?'),                                 # Question Mark
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
    
    def __init__(self, code):
        # Build the combined regex with named groups
        tok_regex = '|'.join(f'(?P<{name}>{pattern})' for name, pattern in self.PRIMITIVE_TOKENS)
        self.tokens = []
        
        for mo in re.finditer(tok_regex, code):
            kind = mo.lastgroup
            value = mo.group()
            if kind == 'WHITESPACE' or kind == 'COMMENT':
                continue
            print(kind, value, sep=" ")
            self.tokens.append((kind, value))
    
    def __str__(self):
        return ", ".join(str(token) for token in self.tokens)
    
    def __iter__(self) -> list:
        return self.tokens.__iter__()

def parse(token_stream: Lexer) -> dict:
    tree = {}
    for i, token in enumerate(token_stream):
       pass
            