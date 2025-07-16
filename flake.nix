{
  description = "Rust development environment with nightly toolchain";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    ...
  }: {
    devShell.x86_64-linux = let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
        overlays = [rust-overlay.overlays.default];
      };

      rust = pkgs.rust-bin.selectLatestNightlyWith (
        toolchain:
          toolchain.default.override {
            extensions = ["rust-src"];
          }
      );
    in
      pkgs.mkShell {
        buildInputs = [
          rust
          pkgs.rust-analyzer
          pkgs.cargo-show-asm
          pkgs.cargo-flamegraph
        ];
      };
  };
}
