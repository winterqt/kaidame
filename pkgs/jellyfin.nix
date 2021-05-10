manifest:
{ lib, stdenv, fetchurl, makeWrapper, dotnetCorePackages }:

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
    makeWrapper ${dotnetCorePackages.aspnetcore_5_0}/bin/dotnet $out/bin/jellyfin --add-flags $out/jellyfin/jellyfin.dll
  '';
}
