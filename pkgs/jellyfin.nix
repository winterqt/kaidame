manifest:
{ lib, stdenv, fetchurl, makeWrapper, autoPatchelfHook, dotnetCorePackages, fontconfig }:

let
  rid = {
    "x86_64-linux" = "linux-x64";
  }.${stdenv.hostPlatform.system};
  nativeLibsToRemove = lib.remove rid [
    "alpine-x64"
    "linux-arm"
    "linux-arm64"
    "linux-armel"
    "linux-mips64"
    "linux-musl-x64"
    "linux-x64"
    "linux-x86"
    "osx"
    "osx-x64"
    "win-arm"
    "win-arm64"
    "win-x64"
    "win-x86"
  ];
in

stdenv.mkDerivation {
  name = "jellyfin";
  version = manifest.version;

  src = fetchurl {
    name = "jellyfin.tar.gz";
    inherit (manifest) url sha256;
  };

  nativeBuildInputs = [ makeWrapper autoPatchelfHook ];

  buildInputs = [ stdenv.cc.cc fontconfig ];

  installPhase = ''
    mkdir -p $out/bin $out/jellyfin
    cp -r * $out/jellyfin
    ${lib.concatMapStringsSep "\n" (rid: ''
      rm -r $out/jellyfin/runtimes/${rid}
    '') nativeLibsToRemove}
    makeWrapper ${dotnetCorePackages.aspnetcore_5_0}/bin/dotnet $out/bin/jellyfin --add-flags $out/jellyfin/jellyfin.dll
  '';
}
