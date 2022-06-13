{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, flake-utils, naersk, flake-compat }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = nixpkgs.legacyPackages."${system}";
        naersk-lib = naersk.lib."${system}";
      in
        rec {
          # `nix build`
          packages.leetcode-rs = naersk-lib.buildPackage {
            pname = "leetcode-rs";
            root = ./.;
          };
          defaultPackage = packages.leetcode-rs;

          # `nix run`
          # apps.hello-world = flake-utils.lib.mkApp {
          #   drv = packages.hello-world;
          # };
          # defaultApp = apps.hello-world;

          # `nix develop`
          devShell = pkgs.mkShell {
            nativeBuildInputs = with pkgs; [ rustc cargo ];
          };
        }
    );
}
