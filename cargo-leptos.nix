{ stdenv, rustPlatform, fetchFromGitHub, lib, pkg-config, openssl }:

rustPlatform.buildRustPackage rec {
  pname = "cargo-leptos";
  version = "0.1.7";
  buildFeatures = [ "no_downloads" ]; # cargo-leptos will try to download Ruby and other things without this feature

  src = fetchFromGitHub {
    owner = "leptos-rs";
    repo = pname;
    rev = version;
    hash = "sha256-Z7JRTsB3krXAKHbdezaTjR6mUQ07+e4pYtpaMLuoR8I=";
  };

  cargoSha256 = "sha256-MqEErweIHHF8w7WANfh8OpzvS774aIfcfkEOwEofSqw=";

  nativeBuildInputs = [ pkg-config openssl ];

  buildInputs = [ openssl pkg-config ];

  doCheck = false; # integration tests depend on changing cargo config
}
