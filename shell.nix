let
  nixpkgs = import <nixpkgs> {};
in
  with nixpkgs;

  stdenv.mkDerivation {
    name = "adventofcode-dev-env";
    buildInputs = [
      cargo
      rustc
      gnuplot
    ];
    OPENSSL_DEV=openssl.dev;
  }