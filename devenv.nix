{ pkgs, config, ... }:
{
  languages.rust.enable = true;

  packages = with pkgs; [
    libadwaita
    pkg-config
    gtk4
  ];

  outputs.app = config.languages.rust.import ./. {
    crateOverrides = pkgs.defaultCrateOverrides // {
      libadwaita-sys = _: {
        nativeBuildInputs = [ pkgs.pkg-config ];
        buildInputs = [ pkgs.libadwaita ];
      };

      tictacini-gusini = _: {
        nativeBuildInputs = [ pkgs.wrapGAppsHook4 ];
        buildInputs = [ pkgs.gtk4 pkgs.libadwaita ];
      };
    };
  };
}