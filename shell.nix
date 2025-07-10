{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
  shellHook = "echo Activated the dev shell";
  packages = with pkgs; [ rustup rustc cargo ];
}
