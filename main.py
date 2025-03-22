from nix_parser import Position, Span

print(isinstance(Span(Position(1, 2), Position(1, 3)), Span))