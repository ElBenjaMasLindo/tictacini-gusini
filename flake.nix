{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-26.05";

  outputs = { nixpkgs }: {
    packages = nixpkgs.lib.genAttrs [ "x86_64-linux" "aarch64-linux" ] (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in {
        default = pkgs.rustPlatform.buildRustPackage {
          pname = "tictacini-gusini";
          version = "0.1.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;

          nativeBuildInputs = with pkgs; [
            pkg-config
            wrapGAppsHook4
            copyDesktopItems
          ];
          buildInputs = with pkgs; [ gtk4 libadwaita ];

          desktopItems = [
            (pkgs.makeDesktopItem {
              name = "com.elbenjamaslindo.tictacini-gusini";
              desktopName = "Tictacini Gusini";
              exec = "tictacini-gusini";
              icon = "com.elbenjamaslindo.tictacini-gusini";
              startupWMClass = "com.elbenjamaslindo.tictacini-gusini";
              categories = [ "Game" ];
            })
          ];

          postInstall = ''
            install -Dm644 data/icons/hicolor/512x512/apps/com.elbenjamaslindo.tictacini-gusini.png \
              $out/share/icons/hicolor/512x512/apps/com.elbenjamaslindo.tictacini-gusini.png
          '';
        };
      });
  };
}