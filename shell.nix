{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = [
    pkgs.cargo
    pkgs.rustc
    pkgs.gcc

    # keep this line if you use bash
    pkgs.bashInteractive
  ];
}
