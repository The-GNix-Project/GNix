"""
Defines Nix Grammar and classes need to Parse a Nix script

Contains:
class       Token           Most basic building of the NIX programming language, refer to parser.lexer.py for all different token definitions and types 
class       TokenStream     A TokenStream is a list of tokens, this class is designed to behave just like a list
list        NIX_KEYWORDS    Definition of all Keywords in nix
list        OPERATORS       Definition of all operators in nix
list        PRIMITIVE_TYPES Definition of all primitive (int, str, bool, etc.) types in nix
"""

from typing import Any, List, Self
from dataclasses import dataclass, field

# MARK: UTIL CLASSES
@dataclass(frozen=True, order=True)
class Token:
    """Most basic building of the NIX programming language, refer to parser.lexer.py for all different token definitions and types"""
    type: str
    content: str
    def __str__(self):return f"{self.type} {self.content} "

@dataclass
class TokenStream:
    """A TokenStream is a list of tokens, this class is designed to behave just like a list"""
    stream: List[Token] = field(default_factory=list)
    type: str = "STREAM"
    
    # Make TokenStream behave like a list
    def append(self, val: Token|Self):
        """Appends the token stream with a Token or TokenStream

        :param val: _description_
        :type val: Token | TokenStream
        """
        assert isinstance(val, Token) or isinstance(val, TokenStream), TypeError(f"Non Token/TokenStream object: {val} attempted to be appended to TokenStream")
        self.stream.append(val)
    def pop(self, index: int) -> Token: 
        """Remove and return item in TokenStream at index (default last).

        Raises IndexError if list is empty or index is out of range.

        :param index: index to remove
        :type index: int
        :return: returns the Token removed
        :rtype: Token
        """
        return self.stream.pop(index)
    def __iter__(self): return self.stream.__iter__()
    def __getitem__(self, index)->Token: return self.stream[index]
    def __len__(self): return self.stream.__len__()
    def __add__(self, x:Self)->Self: self.stream += x; return self

# MARK: NIX GRAMMAR

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

OPERATORS = ['ATTR_SELECT', 
        'HAS_ATTR',    
        'LIST_CONCAT', 
        'MUL',         
        'DIV',         
        'SUB',         
        'ADD',         
        'LOGICAL_NOT', 
        'UPDATE',      
        'LT',          
        'LTE',         
        'GT',          
        'GTE',         
        'EQ_OP',       
        'NEQ_OP',      
        'LOGICAL_AND', 
        'LOGICAL_OR',  
        'IMPLIES',     
        'PIPE_LEFT',   
        'PIPE_RIGHT'
] 

PRIMITIVE_TYPES = [
    "INTEGER",
    "FLOAT",
    "STRING",
    "BOOL",
    "NULL",
    "PATH"
]
