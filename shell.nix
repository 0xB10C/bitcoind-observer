{ pkgs ? import <nixpkgs> {}}:
pkgs.mkShell {
    nativeBuildInputs = [
      pkgs.linuxPackages.bcc
      pkgs.cargo
      pkgs.rustc
      pkgs.rustfmt
    ];
}
