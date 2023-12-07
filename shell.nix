let
  pkgs = import <nixpkgs> {};
in
  with pkgs;
    mkShell {
      packages = [
        cargo
        rustc
        rustfmt
        rust-analyzer-unwrapped

        perlPackages.perl
        perlPackages.LWPProtocolHttps
      ];
      env = {
        RUST_SRC_PATH = "${rustPlatform.rustLibSrc}";
      };
    }
