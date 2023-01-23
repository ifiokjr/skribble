{ pkgs ? import (fetchTarball "https://github.com/NixOS/nixpkgs/archive/9d351dcac0372f4a517d5433537e179ce455402f.tar.gz") {} }:

pkgs.mkShell {
  buildInputs = [
    pkgs.which
    pkgs.htop
    pkgs.zlib
  ];

  shellHook = ''
    echo hello
  '';

  MY_ENV = "world";
}