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
    shellHook = ''
      export LIBCLANG_PATH="${pkgs.llvmPackages.libclang}/lib";
    '';
  };
in shell
