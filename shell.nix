let
  pkgs = import <nixpkgs> {};
in
  with pkgs;
    mkShellNoCC {
      packages = [
        cargo
        rustc
        rustfmt
        rust-analyzer
      ];
    }
