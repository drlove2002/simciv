{
  description = "Rust package";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
  }: let
    system = "x86_64-linux";
    overlays = [(import rust-overlay)];
    pkgs = import nixpkgs {
      inherit system overlays;
    };
  in {
    devShells.${system}.default = pkgs.mkShell {
      buildInputs = with pkgs; [
        gcc
        # Use rust-bin for more control over toolchains and targets
        (rust-bin.stable.latest.default.override {
          extensions = ["rust-src" "rust-analyzer" "clippy"];
          targets = ["x86_64-unknown-linux-musl"];
        })
        protobuf
      ];
    };
  };
}
