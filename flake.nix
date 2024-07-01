{
  description = "A devShell example";

  inputs = {
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      inputs.nixpkgs.follows = "nixpkgs";
      url = "github:oxalica/rust-overlay";
    };
  };

  outputs = {
    self,
    crane,
    flake-utils,
    nixpkgs,
    rust-overlay,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [(import rust-overlay)];
        };

        rust = pkgs.rust-bin.stable.latest;
        rustToolchain = rust.default.override {
          extensions = [
            # include source for LSP
            "rust-src"
          ];
        };
        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

        src = craneLib.cleanCargoSource (craneLib.path ./.);

        commonArgs = {
          inherit src;
          srtictDeps = true;
        };

        # Build *just* the cargo dependencies, so we can reuse
        # all of that work (e.g. via cachix) when running in CI
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        teleport = craneLib.buildPackage (commonArgs
          // {
            inherit cargoArtifacts;
          });
      in {
        checks = {
          # Build the crate as part of `nix flake check` for convenience
          inherit teleport;

          # Run clippy (and deny all warnings) on the crate source,
          # again, resuing the dependency artifacts from above.
          #
          # Note that this is done as a separate derivation so that
          # we can block the CI if there are issues here, but not
          # prevent downstream consumers from building our crate by itself.
          teleport-clippy = craneLib.cargoClippy (commonArgs
            // {
              inherit cargoArtifacts;
              cargoClippyExtraArgs = "--all-targets -- --deny warnings";
            });

          # Check formatting
          teleport-fmt = craneLib.cargoFmt {
            inherit src;
          };
        };
        packages.default = teleport;
        devShells.default = craneLib.devShell {
          packages = with pkgs; [
            eza
            fd
            rust.rust-analyzer
          ];
        };
      }
    );
}
