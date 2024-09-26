{ sources ? import ./nix/sources.nix
, pkgs ? import sources.nixpkgs {}
}:

let
  rust-toolchain = pkgs.symlinkJoin {
    name = "rust-toolchain";
    paths = [pkgs.rustc pkgs.cargo pkgs.rustPlatform.rustcSrc];
  };
in pkgs.mkShell {
  buildInputs = with pkgs; [
    rustc
    cargo
    rustfmt
    rust-analyzer
    clippy
    rust-toolchain
  ];

  RUST_BACKTRACE = 1;
}
