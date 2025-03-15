import json

import nix_parser  # Use your actual module name (e.g., `nixel_bindings`)
nix_script = """
{ pkgs ? import <nixpkgs> {} }:
let
  # Choose a specific LLVM version; here we choose LLVM 12 as an example.
  llvm = pkgs.llvmPackages_12;
in
pkgs.mkShell {
  buildInputs = [
    pkgs.gcc                         # Use GCC/g++ for compiling C/C++ code.
    llvm.llvm                        # Provides llvm-config.
    llvm.libclang                    # Provides libclang shared libraries.
    pkgs.python3
    pkgs.maturin
    pkgs.glibc.dev                   # Provides glibc headers (e.g. stddef.h).
    pkgs.nixel                       # nixel from nixpkgs.
  ];

  shellHook = ''
    export CC=gcc
    export CXX=g++
    # Provide llvm-config and libclang paths.
    export LLVM_CONFIG_PATH="${llvm.llvm}/bin/llvm-config"
    export LIBCLANG_PATH="${llvm.libclang.lib}/lib"
    # Explicitly add glibc development headers as system include directories.
    export CFLAGS="-isystem ${pkgs.glibc.dev}/include"
    export CXXFLAGS="-isystem ${pkgs.glibc.dev}/include"
    echo "Environment set up with GCC and glibc includes."
    echo "LLVM_CONFIG_PATH: $LLVM_CONFIG_PATH"
    echo "LIBCLANG_PATH: $LIBCLANG_PATH"

    VENV=venv

    if test ! -d $VENV; then
      python3.12 -m venv $VENV
    fi
    source ./$VENV/bin/activate
    echo Virtual Environment Activated!
  '';
}

"""
json_output = json.loads(nix_parser.parse_nix(nix_script))
print(json.dumps(json_output["expression"]))