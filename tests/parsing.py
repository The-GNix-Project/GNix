from src.parser.lexer import lex
from src.parser.parsing import parse_brackets
from src.parser.grammar import Token, TokenStream

def test_lexer():
    # Check token streams/basic nix expressions are lexed correctly
    assert lex("myVar = 5;") == TokenStream(stream=[Token(type='IDENTIFIER', content='myVar'),
                                                    Token(type='EQ', content='='),
                                                    Token(type='INTEGER', content='5'),
                                                    Token(type='SEMICOLON', content=';')], type='STREAM')
    # Check composed data types are lexed as expected
    assert lex('myVar = [1 "string" [1 2 ]  ];') == TokenStream(stream=[Token(type='IDENTIFIER', content='myVar'),
                                                                        Token(type='EQ', content='='),
                                                                        Token(type="LSPAREN", content="["),
                                                                        Token(type='INTEGER', content='1'),
                                                                        Token(type="STRING", content='"string"'),
                                                                        Token(type="LSPAREN", content="["),
                                                                        Token(type='INTEGER', content='1'),
                                                                        Token(type='INTEGER', content='2'),
                                                                        Token(type="RSPAREN", content="]"),
                                                                        Token(type="RSPAREN", content="]"),
                                                                        Token(type='SEMICOLON', content=';')], type='STREAM')
    # Check strings could be defined with " or '
    assert lex("''string''") == TokenStream(stream=[Token(type='STRING', content="''string''")])
    assert lex('"string"') == TokenStream(stream=[Token(type='STRING', content='"string"')])
    assert lex("3124") == TokenStream(stream=[Token(type="INTEGER", content="3124")])
    assert lex("31.24") == TokenStream(stream=[Token(type="FLOAT", content="31.24")])
    
    
    # Check more complex Nix scripts are lexed as expected
    lexed = ""
    with open("tests/static/basix-nix.nix", 'r') as file:
        lexed = lex(file.read())        
    assert lexed == TokenStream(stream=[Token(type='LBRACE', content='{'), 
                                        Token(type='IDENTIFIER', content='config'), 
                                        Token(type='COMMA', content=','), 
                                        Token(type='IDENTIFIER', content='pkgs'), 
                                        Token(type='COMMA', content=','), 
                                        Token(type='IDENTIFIER', content='lib'), 
                                        Token(type='COMMA', content=','), 
                                        Token(type='IDENTIFIER', content='inputs'), 
                                        Token(type='COMMA', content=','), 
                                        Token(type='IDENTIFIER', content='instring'), 
                                        Token(type='HAS_ATTR', content='?'), 
                                        Token(type='STRING', content='"myVal"'), 
                                        Token(type='COMMA', content=','), 
                                        Token(type='ELLIPSE', content='...'), 
                                        Token(type='RBRACE', content='}'), 
                                        Token(type='COLON', content=':'), 
                                        Token(type='LBRACE', content='{'), 
                                        Token(type='IDENTIFIER', content='experimental-features'), 
                                        Token(type='EQ', content='='), 
                                        Token(type='LSPAREN', content='['), 
                                        Token(type='STRING', content='"nix-command"'), 
                                        Token(type='STRING', content='"flakes"'), 
                                        Token(type='LSPAREN', content='['), 
                                        Token(type='STRING', content='"test"'), 
                                        Token(type='INTEGER', content='2'), 
                                        Token(type='RSPAREN', content=']'), 
                                        Token(type='LBRACE', content='{'), 
                                        Token(type='IDENTIFIER', content='a'), 
                                        Token(type='EQ', content='='), 
                                        Token(type='INTEGER', content='2'), 
                                        Token(type='SEMICOLON', content=';'), 
                                        Token(type='IDENTIFIER', content='b'), 
                                        Token(type='EQ', content='='), 
                                        Token(type='INTEGER', content='3'), 
                                        Token(type='RBRACE', content='}'), 
                                        Token(type='RSPAREN', content=']'), 
                                        Token(type='SEMICOLON', content=';'), 
                                        Token(type='IDENTIFIER', content='imports'), 
                                        Token(type='EQ', content='='), 
                                        Token(type='LSPAREN', content='['), 
                                        Token(type='PATH', content='./hardware-configuration.nix'), 
                                        Token(type='RSPAREN', content=']'), 
                                        Token(type='SEMICOLON', content=';'), 
                                        Token(type='IDENTIFIER', content='myVar'), 
                                        Token(type='EQ', content='='), 
                                        Token(type='INTEGER', content='4'), 
                                        Token(type='SEMICOLON', content=';'), 
                                        Token(type='RBRACE', content='}')], type='STREAM')
    nixos_config_lexed = TokenStream(stream=[Token(type='LBRACE', content='{'), 
                                           Token(type='IDENTIFIER', content='config'), 
                                           Token(type='COMMA', content=','), 
                                           Token(type='IDENTIFIER', content='pkgs'), 
                                           Token(type='COMMA', content=','), 
                                           Token(type='IDENTIFIER', content='lib'), 
                                           Token(type='COMMA', content=','), 
                                           Token(type='IDENTIFIER', content='inputs'), 
                                           Token(type='COMMA', content=','), 
                                           Token(type='ELLIPSE', content='...'), 
                                           Token(type='RBRACE', content='}'), 
                                           Token(type='COLON', content=':'), 
                                           Token(type='LBRACE', content='{'), 
                                           Token(type='IDENTIFIER', content='nix'), 
                                           Token(type='ATTR_SELECT', content='.'), 
                                           Token(type='IDENTIFIER', content='settings'), 
                                           Token(type='ATTR_SELECT', content='.'), 
                                           Token(type='IDENTIFIER', content='experimental-features'), 
                                           Token(type='EQ', content='='), 
                                           Token(type='LSPAREN', content='['), 
                                           Token(type='STRING', content='"nix-command"'), 
                                           Token(type='STRING', content='"flakes"'), 
                                           Token(type='RSPAREN', content=']'), 
                                           Token(type='SEMICOLON', content=';'), 
                                           Token(type='IDENTIFIER', content='imports'), 
                                           Token(type='EQ', content='='), 
                                           Token(type='LSPAREN', content='['), 
                                           Token(type='PATH', content='./hardware-configuration.nix'), 
                                           Token(type='RSPAREN', content=']'), 
                                           Token(type='SEMICOLON', content=';'), 
                                           Token(type='IDENTIFIER', content='boot'), 
                                           Token(type='ATTR_SELECT', content='.'), 
                                           Token(type='IDENTIFIER', content='loader'), 
                                           Token(type='ATTR_SELECT', content='.'), 
                                           Token(type='IDENTIFIER', content='systemd-boot'), 
                                           Token(type='ATTR_SELECT', content='.'), 
                                           Token(type='IDENTIFIER', content='enable'), 
                                           Token(type='EQ', content='='), 
                                           Token(type='BOOL', content='true'), 
                                           Token(type='SEMICOLON', content=';'), 
                                           Token(type='IDENTIFIER', content='boot'), 
                                           Token(type='ATTR_SELECT', content='.'), 
                                           Token(type='IDENTIFIER', content='loader'), 
                                           Token(type='ATTR_SELECT', content='.'), 
                                           Token(type='IDENTIFIER', content='efi'), 
                                           Token(type='ATTR_SELECT', content='.'), 
                                           Token(type='IDENTIFIER', content='canTouchEfiVariables'), 
                                           Token(type='EQ', content='='), 
                                           Token(type='BOOL', content='true'), 
                                           Token(type='SEMICOLON', content=';'), 
                                           Token(type='IDENTIFIER', content='networking'), 
                                           Token(type='ATTR_SELECT', content='.'), 
                                           Token(type='IDENTIFIER', content='hostName'), 
                                           Token(type='EQ', content='='), 
                                           Token(type='STRING', content='"nix"'), 
                                           Token(type='SEMICOLON', content=';'), 
                                           Token(type='IDENTIFIER', content='networking'), 
                                           Token(type='ATTR_SELECT', content='.'), 
                                           Token(type='IDENTIFIER', content='networkmanager'), 
                                           Token(type='ATTR_SELECT', content='.'), 
                                           Token(type='IDENTIFIER', content='enable'), 
                                           Token(type='EQ', content='='), 
                                           Token(type='BOOL', content='true'), 
                                           Token(type='SEMICOLON', content=';'), 
                                           Token(type='IDENTIFIER', content='users'), 
                                           Token(type='ATTR_SELECT', content='.'), 
                                           Token(type='IDENTIFIER', content='users'), 
                                           Token(type='ATTR_SELECT', content='.'), 
                                           Token(type='IDENTIFIER', content='user'), 
                                           Token(type='EQ', content='='), 
                                           Token(type='LBRACE', content='{'), 
                                           Token(type='IDENTIFIER', content='isNormalUser'), 
                                           Token(type='EQ', content='='), 
                                           Token(type='BOOL', content='true'), 
                                           Token(type='SEMICOLON', content=';'), 
                                           Token(type='IDENTIFIER', content='description'), 
                                           Token(type='EQ', content='='), 
                                           Token(type='STRING', content='"A User"'), 
                                           Token(type='SEMICOLON', content=';'), 
                                           Token(type='IDENTIFIER', content='extraGroups'), 
                                           Token(type='EQ', content='='), 
                                           Token(type='LSPAREN', content='['), 
                                           Token(type='STRING', content='"networkmanager"'), 
                                           Token(type='STRING', content='"wheel"'), 
                                           Token(type='RSPAREN', content=']'), 
                                           Token(type='SEMICOLON', content=';'), 
                                           Token(type='IDENTIFIER', content='packages'), 
                                           Token(type='EQ', content='='), 
                                           Token(type='KEYWORD', content='with'), 
                                           Token(type='IDENTIFIER', content='pkgs'), 
                                           Token(type='SEMICOLON', content=';'), 
                                           Token(type='LSPAREN', content='['), 
                                           Token(type='IDENTIFIER', content='home-manager'), 
                                           Token(type='RSPAREN', content=']'), 
                                           Token(type='SEMICOLON', content=';'), 
                                           Token(type='RBRACE', content='}'), 
                                           Token(type='SEMICOLON', content=';'), 
                                           Token(type='IDENTIFIER', content='nixpkgs'), 
                                           Token(type='ATTR_SELECT', content='.'), 
                                           Token(type='IDENTIFIER', content='config'), 
                                           Token(type='ATTR_SELECT', content='.'), 
                                           Token(type='IDENTIFIER', content='allowUnfree'), 
                                           Token(type='EQ', content='='), 
                                           Token(type='BOOL', content='true'), 
                                           Token(type='SEMICOLON', content=';'), 
                                           Token(type='IDENTIFIER', content='nixpkgs'), 
                                           Token(type='ATTR_SELECT', content='.'), 
                                           Token(type='IDENTIFIER', content='config'), 
                                           Token(type='ATTR_SELECT', content='.'), 
                                           Token(type='IDENTIFIER', content='pulseaudio'), 
                                           Token(type='EQ', content='='), 
                                           Token(type='BOOL', content='true'), 
                                           Token(type='SEMICOLON', content=';'), 
                                           Token(type='IDENTIFIER', content='environment'), 
                                           Token(type='ATTR_SELECT', content='.'), 
                                           Token(type='IDENTIFIER', content='systemPackages'), 
                                           Token(type='EQ', content='='), 
                                           Token(type='KEYWORD', content='with'), 
                                           Token(type='IDENTIFIER', content='pkgs'), 
                                           Token(type='SEMICOLON', content=';'), 
                                           Token(type='LSPAREN', content='['), 
                                           Token(type='IDENTIFIER', content='git'), 
                                           Token(type='IDENTIFIER', content='vim'), 
                                           Token(type='IDENTIFIER', content='findutils'), 
                                           Token(type='IDENTIFIER', content='wget'), 
                                           Token(type='IDENTIFIER', content='findutils'), 
                                           Token(type='ATTR_SELECT', content='.'), 
                                           Token(type='IDENTIFIER', content='locate'), 
                                           Token(type='IDENTIFIER', content='clinfo'), 
                                           Token(type='IDENTIFIER', content='system-config-printer'), 
                                           Token(type='IDENTIFIER', content='avahi'), 
                                           Token(type='IDENTIFIER', content='gnumake'), 
                                           Token(type='RSPAREN', content=']'), 
                                           Token(type='SEMICOLON', content=';'), 
                                           Token(type='IDENTIFIER', content='fonts'), 
                                           Token(type='ATTR_SELECT', content='.'), 
                                           Token(type='IDENTIFIER', content='packages'), 
                                           Token(type='EQ', content='='), 
                                           Token(type='LSPAREN', content='['), 
                                           Token(type='RSPAREN', content=']'), 
                                           Token(type='LIST_CONCAT', content='++'), 
                                           Token(type='IDENTIFIER', content='builtins'), 
                                           Token(type='ATTR_SELECT', content='.'), 
                                           Token(type='IDENTIFIER', content='filter'), 
                                           Token(type='IDENTIFIER', content='lib'), 
                                           Token(type='ATTR_SELECT', content='.'), 
                                           Token(type='IDENTIFIER', content='attrsets'), 
                                           Token(type='ATTR_SELECT', content='.'), 
                                           Token(type='IDENTIFIER', content='isDerivation'), 
                                           Token(type='LPAREN', content='('), 
                                           Token(type='IDENTIFIER', content='builtins'), 
                                           Token(type='ATTR_SELECT', content='.'), 
                                           Token(type='IDENTIFIER', content='attrValues'), 
                                           Token(type='IDENTIFIER', content='pkgs'), 
                                           Token(type='ATTR_SELECT', content='.'), 
                                           Token(type='IDENTIFIER', content='nerd-fonts'), 
                                           Token(type='RPAREN', content=')'), 
                                           Token(type='SEMICOLON', content=';'), 
                                           Token(type='IDENTIFIER', content='programs'), 
                                           Token(type='EQ', content='='), 
                                           Token(type='LBRACE', content='{'), 
                                           Token(type='IDENTIFIER', content='thunar'), 
                                           Token(type='ATTR_SELECT', content='.'), 
                                           Token(type='IDENTIFIER', content='enable'), 
                                           Token(type='EQ', content='='), 
                                           Token(type='BOOL', content='true'), 
                                           Token(type='SEMICOLON', content=';'), 
                                           Token(type='IDENTIFIER', content='dconf'), 
                                           Token(type='ATTR_SELECT', content='.'), 
                                           Token(type='IDENTIFIER', content='enable'), 
                                           Token(type='EQ', content='='), 
                                           Token(type='BOOL', content='true'), 
                                           Token(type='SEMICOLON', content=';'), 
                                           Token(type='IDENTIFIER', content='sway'), 
                                           Token(type='ATTR_SELECT', content='.'), 
                                           Token(type='IDENTIFIER', content='enable'), 
                                           Token(type='EQ', content='='), 
                                           Token(type='BOOL', content='true'), 
                                           Token(type='SEMICOLON', content=';'), 
                                           Token(type='IDENTIFIER', content='kdeconnect'), 
                                           Token(type='ATTR_SELECT', content='.'), 
                                           Token(type='IDENTIFIER', content='enable'), 
                                           Token(type='EQ', content='='), 
                                           Token(type='BOOL', content='true'), 
                                           Token(type='SEMICOLON', content=';'), 
                                           Token(type='IDENTIFIER', content='partition-manager'), 
                                           Token(type='ATTR_SELECT', content='.'), 
                                           Token(type='IDENTIFIER', content='enable'), 
                                           Token(type='EQ', content='='), 
                                           Token(type='BOOL', content='true'), 
                                           Token(type='SEMICOLON', content=';'), 
                                           Token(type='RBRACE', content='}'), 
                                           Token(type='SEMICOLON', content=';'), 
                                           Token(type='RBRACE', content='}')], type='STREAM')
    with open("tests/static/example-nixos-config.nix", 'r') as file:
        lexed = lex(file.read())
    assert lexed == nixos_config_lexed        

    # INTEGER
    assert lex("3124") == TokenStream(stream=[Token(type="INTEGER", content="3124")])
    
    # STRING (both single and double quoted)
    assert lex("''hello world''") == TokenStream(stream=[Token(type="STRING", content="''hello world''")])
    assert lex('"hello world"') == TokenStream(stream=[Token(type="STRING", content='"hello world"')])
    
    # BOOL
    assert lex("true") == TokenStream(stream=[Token(type="BOOL", content="true")])
    assert lex("false") == TokenStream(stream=[Token(type="BOOL", content="false")])
    
    # NULL
    assert lex("null") == TokenStream(stream=[Token(type="NULL", content="null")])
    
    # FLOAT
    assert lex("31.24") == TokenStream(stream=[Token(type="FLOAT", content="31.24")])
    # (Also tests scientific notation if needed, e.g., "1e3")
    assert lex("1e3") == TokenStream(stream=[Token(type="FLOAT", content="1e3")])
    
    # PATH (assuming paths start with './' or '/')
    assert lex("./my-file") == TokenStream(stream=[Token(type="PATH", content="./my-file")])
    assert lex("/usr/bin") == TokenStream(stream=[Token(type="PATH", content="/usr/bin")])
    
    # KEYWORD (adjust according to your KEYWORD_REGEX; e.g., assuming "if" is reserved)
    assert lex("if") == TokenStream(stream=[Token(type="KEYWORD", content="if")])
    
    # IDENTIFIER
    assert lex("variableName") == TokenStream(stream=[Token(type="IDENTIFIER", content="variableName")])
    
    # Punctuation tokens
    assert lex("{") == TokenStream(stream=[Token(type="LBRACE", content="{")])
    assert lex("}") == TokenStream(stream=[Token(type="RBRACE", content="}")])
    assert lex("(") == TokenStream(stream=[Token(type="LPAREN", content="(")])
    assert lex(")") == TokenStream(stream=[Token(type="RPAREN", content=")")])
    assert lex("[") == TokenStream(stream=[Token(type="LSPAREN", content="[")])
    assert lex("]") == TokenStream(stream=[Token(type="RSPAREN", content="]")])
    assert lex(",") == TokenStream(stream=[Token(type="COMMA", content=",")])
    assert lex("=") == TokenStream(stream=[Token(type="EQ", content="=")])
    assert lex(":") == TokenStream(stream=[Token(type="COLON", content=":")])
    assert lex(";") == TokenStream(stream=[Token(type="SEMICOLON", content=";")])
    assert lex("...") == TokenStream(stream=[Token(type="ELLIPSE", content="...")])
    
    # Test that WHITESPACE is ignored by combining tokens with spaces
    assert lex("3124   31.24") == TokenStream(stream=[
        Token(type="INTEGER", content="3124"),
        Token(type="FLOAT", content="31.24")
    ])
    
    # Test that COMMENT is ignored (comment starts with #)
    assert lex("3124 # this is a comment\n31.24") == TokenStream(stream=[
        Token(type="INTEGER", content="3124"),
        Token(type="FLOAT", content="31.24")
    ])
    
    # Operators
    assert lex(".") == TokenStream(stream=[Token(type="ATTR_SELECT", content=".")])
    assert lex("?") == TokenStream(stream=[Token(type="HAS_ATTR", content="?")])
    assert lex("++") == TokenStream(stream=[Token(type="LIST_CONCAT", content="++")])
    assert lex("*") == TokenStream(stream=[Token(type="MUL", content="*")])
    assert lex("/") == TokenStream(stream=[Token(type="DIV", content="/")])
    assert lex("-") == TokenStream(stream=[Token(type="SUB", content="-")])
    assert lex("+") == TokenStream(stream=[Token(type="ADD", content="+")])
    assert lex("!") == TokenStream(stream=[Token(type="LOGICAL_NOT", content="!")])
    assert lex("//") == TokenStream(stream=[Token(type="UPDATE", content="//")])
    assert lex("<") == TokenStream(stream=[Token(type="LT", content="<")])
    assert lex("<=") == TokenStream(stream=[Token(type="LTE", content="<=")])
    assert lex(">") == TokenStream(stream=[Token(type="GT", content=">")])
    assert lex(">=") == TokenStream(stream=[Token(type="GTE", content=">=")])
    assert lex("==") == TokenStream(stream=[Token(type="EQ_OP", content="==")])
    assert lex("!=") == TokenStream(stream=[Token(type="NEQ_OP", content="!=")])
    assert lex("&&") == TokenStream(stream=[Token(type="LOGICAL_AND", content="&&")])
    assert lex("||") == TokenStream(stream=[Token(type="LOGICAL_OR", content="||")])
    assert lex("->") == TokenStream(stream=[Token(type="IMPLIES", content="->")])
    assert lex("|>") == TokenStream(stream=[Token(type="PIPE_LEFT", content="|>")])
    assert lex("<|") == TokenStream(stream=[Token(type="PIPE_RIGHT", content="<|")])

def test_parse_brackets():
    lexed = lex("{{[]}[]}")
    assert parse_brackets(lex("{{[]}[]}")) == TokenStream(stream=[TokenStream(stream=[Token(type='LBRACE', content='{'),
                                                                                    TokenStream(stream=[Token(type='LBRACE', content='{'), 
                                                                                                        TokenStream(stream=[Token(type='LSPAREN', content='['),
                                                                                                                            Token(type='RSPAREN', content=']')], type='STREAM'
                                                                                                                    ),
                                                                                                        Token(type='RBRACE', content='}')], type='STREAM'
                                                                                                ),
                                                                                    TokenStream(stream=[Token(type='LSPAREN', content='['), 
                                                                                                        Token(type='RSPAREN', content=']')], type='STREAM'
                                                                                                ), 
                                                                                    Token(type='RBRACE', content='}')], type='STREAM'
                                                                                )
                                                                ], type='STREAM')
    assert parse_brackets(lex("{''string''[123]}")) == TokenStream(stream=[TokenStream(stream=[Token(type='LBRACE', content='{'), 
                                                                      Token(type='STRING', content="''string''"),
                                                                      TokenStream(stream=[Token(type='LSPAREN', content='['),
                                                                                          Token(type='INTEGER', content='123'),
                                                                                          Token(type='RSPAREN', content=']')], type='STREAM'
                                                                                  ),
                                                                      Token(type='RBRACE', content='}')], type='STREAM')], type='STREAM')
                                                            
                                                            
                                                            
                                                            
                                                            