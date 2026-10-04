{ pkgs, ... }:

{
  languages.rust.enable = true;

  packages = with pkgs; [
    pkg-config
    gtk4
  ];
}
