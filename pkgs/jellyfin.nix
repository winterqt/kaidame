manifest:
{ lib, buildFHSUserEnv, stdenv, fetchurl, makeWrapper, autoPatchelfHook
, fontconfig, zlib, lttngUst, krb5, icu, openssl }:

stdenv.mkDerivation rec {
  name = "jellyfin";
  version = manifest.version;

  src = fetchurl {
    name = "jellyfin.tar.gz";
    inherit (manifest) url sha256;
  };

  nativeBuildInputs = [ makeWrapper autoPatchelfHook ];

  buildInputs = [ stdenv.cc.cc fontconfig zlib lttngUst krb5 ];

  installPhase = ''
    mkdir -p $out/bin $out/jellyfin
    cp -r * $out/jellyfin
    makeWrapper $out/jellyfin/jellyfin $out/bin/jellyfin --prefix LD_LIBRARY_PATH : "${
      lib.makeLibraryPath [ icu openssl ]
    }"
  '';
}
