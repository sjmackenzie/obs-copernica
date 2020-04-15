{ pkgs ? import <nixpkgs> {} }:
with pkgs;
let
  shell = pkgs.stdenv.mkDerivation {
    name = "shell";
    buildInputs = [
      qt5.full
      qt5.qtcharts
      qt5.qttools
      openssl
      sqlite
      pkgconfig
      clang
      cmake
      rustup
      obs-studio
    ];
  };
in shell
