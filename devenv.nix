{ pkgs, ... }:

{
  # https://devenv.sh/packages/
  packages = [ 
    pkgs.cargo-all-features
    pkgs.cargo-generate
    pkgs.cargo-insta
    pkgs.cargo-make
    pkgs.deno
    pkgs.dprint
    pkgs.fnm
    pkgs.git
    pkgs.rust-analyzer
    pkgs.rustup
    pkgs.trunk
  ];

  difftastic.enable = true;
  devcontainer.enable = true;

  # https://devenv.sh/languages/
  # languages.nix.enable = true;

  # https://devenv.sh/scripts/
  scripts.hello.exec = "echo hello from YO";

  # https://devenv.sh/pre-commit-hooks/
  # pre-commit.hooks.shellcheck.enable = true;

  # https://devenv.sh/processes/
  # processes.ping.exec = "ping example.com";
}
