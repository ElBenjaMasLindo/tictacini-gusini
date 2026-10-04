{ pkgs, ... }:

{
  languages.rust.enable = true;

  packages = with pkgs; [
    libadwaita
    pkg-config
    gtk4
  ];
}
