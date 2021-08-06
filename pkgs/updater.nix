{ lib, stdenv, rustPlatform, pkg-config, openssl, Security }:

rustPlatform.buildRustPackage {
  pname = "kaidame-updater";
  version = (lib.importTOML ../Cargo.toml).package.version;

  src = ../.;
  cargoLock.lockFile = ../Cargo.lock;

  nativeBuildInputs = [ pkg-config ];
  buildInputs = [ openssl ] ++ lib.optionals stdenv.isDarwin [ Security ];
}
