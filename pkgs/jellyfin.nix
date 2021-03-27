manifest:
{ lib, buildFHSUserEnv, stdenv, fetchurl, makeWrapper }:

stdenv.mkDerivation rec {
  name = "jellyfin";
  version = manifest.version;

  src = fetchurl {
    name = "jellyfin.tar.gz";
    inherit (manifest) url sha256;
  };

  nativeBuildInputs = [ makeWrapper ];

  installPhase = ''
    mkdir -p $out/bin $out/jellyfin
    cp -r * $out/jellyfin
  '';
}
