from typing import Any, List, Self
from dataclasses import dataclass, field

# MARK: UTIL CLASSES
@dataclass(frozen=True, order=True)
class Token:
    type: str
    content: str
    def __str__(self):return f"{self.type} {self.content} "

@dataclass
class TokenStream:
    stream: List[Token] = field(default_factory=list)
    type: str = "STREAM"
    
    # Make TokenStream behave like a list
    def append(self, val: Token):
        assert isinstance(val, Token) or isinstance(val, TokenStream), TypeError(f"Non Token/TokenStream object: {val} attempted to be appended to TokenStream")
        self.stream.append(val)
    def pop(self, index): return self.stream.pop(index)
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

def array(token_stream: TokenStream | list):
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