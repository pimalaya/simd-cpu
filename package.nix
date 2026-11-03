# TODO: move this to nixpkgs
# This file aims to be a replacement for the nixpkgs derivation.

{
  buildFeatures ? [ ],
  buildNoDefaultFeatures ? false,
  buildPackages,
  fetchFromGitHub,
  lib,
  rustPlatform,
}:

rustPlatform.buildRustPackage {
  inherit buildNoDefaultFeatures buildFeatures;

  pname = "simd-cpu";
  version = "0.1.0";
  cargoHash = "";

  src = fetchFromGitHub {
    owner = "pimalaya";
    repo = "simd-cpu";
    rev = "v0.1.0";
    hash = "";
  };

  meta = {
    description = "SIMD features of the running CPU as a token for SIMD codecs, no_std and dependency-free";
    homepage = "https://github.com/pimalaya/simd-cpu";
    changelog = "https://github.com/pimalaya/simd-cpu/blob/master/CHANGELOG.md";
    license = [
      lib.licenses.mit
      lib.licenses.asl20
    ];
    maintainers = with lib.maintainers; [ soywod ];
  };
}
