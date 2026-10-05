{
  description = "Tictacini Gusini";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }:
    let
      systems = [ "x86_64-linux" "aarch64-linux" ];
      each = f: nixpkgs.lib.genAttrs systems (system: f nixpkgs.legacyPackages.${system});
      
    in {
      packages = each (pkgs: {
        default = pkgs.rustPlatform.buildRustPackage {
          pname = "tictacini-gusini";
          version = "0.1.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;

          nativeBuildInputs = with pkgs; [ pkg-config wrapGAppsHook4 ];
          buildInputs = with pkgs; [ gtk4 libadwaita ];
        };
      });

      devShells = each (pkgs: {
        default = pkgs.mkShell {
          inputsFrom = [ self.packages.${pkgs.system}.default ];
          packages = with pkgs; [ rustc cargo ];
        };
      });
      
      postInstall = ''
        install -Dm644 data/icons/hicolor/512x512/apps/com.elbenjamaslindo.tictacini-gusini.png \
          $out/share/icons/hicolor/512x512/apps/com.elbenjamaslindo.tictacini-gusini.png
      
        mkdir -p $out/share/applications
        cat > $out/share/applications/com.elbenjamaslindo.tictacini-gusini.desktop <<EOF
        [Desktop Entry]
        Name=Tictacini Gusini
        Exec=tictacini-gusini
        Icon=com.elbenjamaslindo.tictacini-gusini
        Type=Application
        Categories=Game;
        EOF
      '';
    };
}