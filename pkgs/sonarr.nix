manifest:
{ lib, stdenv, fetchurl, makeWrapper, mono, sqlite, libmediainfo }:
stdenv.mkDerivation rec {
  name = "sonarr";
  version = manifest.version;

  src = fetchurl {
    inherit (manifest) url sha256;
  };

  nativeBuildInputs = [ makeWrapper ];

  installPhase = ''
    mkdir -p $out/bin $out/sonarr
    cp -r * $out/sonarr
    makeWrapper ${mono}/bin/mono $out/bin/Sonarr --add-flags $out/sonarr/Sonarr.exe --prefix LD_LIBRARY_PATH : "${
      lib.makeLibraryPath [ sqlite libmediainfo ]
    }"
  '';
}
