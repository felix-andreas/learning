{ stdenv, rustPlatform, fetchFromGitHub, lib, pkg-config, openssl }:

rustPlatform.buildRustPackage rec {
  pname = "cargo-leptos";
  version = "0.1.8";
  buildFeatures = [ "no_downloads" ]; # cargo-leptos will try to download Ruby and other things without this feature

  src = fetchFromGitHub {
    owner = "leptos-rs";
    repo = pname;
    rev = version;
    hash = "sha256-z4AqxvKu9E8GGMj6jNUAAWeqoE/j+6NoAEZWeNZ+1BA=";
  };

  cargoSha256 = "sha256-w/9W4DXbh4G5DZ8IGUz4nN3LEjHhL7HgybHqODMFzHw=";

  nativeBuildInputs = [ pkg-config openssl ];

  buildInputs = [ openssl pkg-config ];

  doCheck = false; # integration tests depend on changing cargo config
}
