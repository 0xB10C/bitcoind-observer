{ pkgs ? import <nixpkgs> {}}:
pkgs.mkShell {
    nativeBuildInputs = [
      pkgs.linuxPackages.bcc
    ];
}
