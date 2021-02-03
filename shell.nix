let
    sources = import ./nix/sources.nix;
  pkgs =
    import sources.nixpkgs { overlays = [ (import sources.nixpkgs-mozilla) ]; };
  rust = pkgs.callPackage ./nix/rust.nix { };
in pkgs.llvmPackages_11.stdenv.mkDerivation {
  name = "clang-nix-shell";

  buildInputs = with pkgs; [
    rust
    sqlite
    sqliteInteractive
    pkg-config
    weechat

    bashInteractive
  ];

  CARGO_NET_GIT_FETCH_WITH_CLI = "true";
  LIBCLANG_PATH = "${pkgs.llvmPackages.libclang}/lib";
  RUST_BACKTRACE = "1";
  RUST_SRC_PATH = "${pkgs.latest.rustChannels.nightly.rust-src}/lib/rustlib/src/rust/library";
  WEECHAT_PLUGIN_FILE = "${pkgs.weechat}/include/weechat/weechat-plugin.h";
}
