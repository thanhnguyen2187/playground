{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = [
    pkgs.stdenv.cc.cc.lib
  ];

  shellHook = ''
    export LD_LIBRARY_PATH="${pkgs.stdenv.cc.cc.lib}/lib:$${LD_LIBRARY_PATH}"
    echo "libstdc++.so.6 path added to LD_LIBRARY_PATH."
  '';
}
