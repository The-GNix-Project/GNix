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
    
    # Make TokenStream behave like a list
    def append(self, val: Token):
        assert isinstance(val, Token) or isinstance(val, TokenStream), TypeError(f"Non Token/TokenStream object: {val} attempted to be appended to TokenStream")
        self.stream.append(val)
    def __iter__(self): return self.stream.__iter__()
    def __getitem__(self, index)->Token:return self.stream[index]
    def __len__(self):return len(self.stream)
    def __add__(self, x:Self)->Self:self.stream += x; return self

# MARK: NIX GRAMMAR
class Grammar:
    name: str
    data: Any

@dataclass(frozen=True, order=True)
class Array(Grammar):
    name = "Array"
    data: list
    
    # Make the Array class act like a python list
    def __iter__(self): return self.data.__iter__()
    def __getitem__(self, index):return self.data[index]
    def __len__(self):return len(self.data)
