{
  description = "A high-quality Rust development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    # Overlays allow us to pull specific versions of the Rust toolchain
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        # This reads your rust-toolchain.toml file so the shell
        # always matches your project's defined version.
        rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

        # Define the system dependencies
        # runtimeDeps = with pkgs; [
        #   openssl
        #   pkg-config
        # ];

        # devTools = with pkgs; [
        #   rustToolchain
        #   cargo-nextest
        #   cargo-llvm-cov
        #   cargo-audit
        #   cargo-expand
        # ];

        # Define the shell derivation
        devShellDerivation = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustToolchain
            pkg-config
            openssl
            cargo-nextest
            cargo-llvm-cov
            cargo-audit
            cargo-expand
          ];
        };
      in
      {
        # This is the modern attribute for 'nix develop'
        # devShells.default = pkgs.mkShell {
        #   buildInputs = runtimeDeps ++ devTools;

        #   # Fixes OpenSSL linking issues on Linux
        #   shellHook = ''
        #     export PKG_CONFIG_PATH="${pkgs.openssl.dev}/lib/pkgconfig"
        #     echo "🦀 High-Quality Rust Dev Environment Loaded"
        #     echo "Build Tools: nextest, llvm-cov, audit"
        #   '';
        # };

        # # Backwards compatibility for 'nix-shell'
        # devShell = self.devShells.${system}.default;

        # Modern Nix (nix develop)
        devShells.default = devShellDerivation;

        # Legacy Nix (nix-shell / older direnv versions)
        devShell = devShellDerivation;
      }
    );
}
