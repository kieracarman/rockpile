{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    rustup
    gcc
    openssl
    pkg-config
    libpqxx
    postgresql
  ];

  shellHook = ''
    export PATH="$HOME/.cargo/bin:$PATH"
    rustup default stable
    rustup component add rustfmt
    rustup component add clippy
  '';
}

