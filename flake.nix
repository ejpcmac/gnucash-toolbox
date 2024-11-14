{
  description = "A CLI toolbox to work with GnuCash databases.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";

    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };

    devshell = {
      url = "github:numtide/devshell";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    git-z = {
      url = "github:ejpcmac/git-z";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, flake-parts, ... }@inputs:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [ inputs.devshell.flakeModule ];
      systems = [ "x86_64-linux" "x86_64-darwin" "aarch64-darwin" ];

      perSystem = { self', inputs', system, ... }:
        let
          overlays = [ (import inputs.rust-overlay) ];
          pkgs = import inputs.nixpkgs { inherit system overlays; };
          rust-toolchain =
            pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        in
        {
          ######################################################################
          ##                             Packages                             ##
          ######################################################################

          packages =
            let
              packageName = "gnucash-toolbox";

              naersk = pkgs.callPackage inputs.naersk {
                cargo = rust-toolchain;
                rustc = rust-toolchain;
              };
            in
            {
              default = self'.packages.${packageName};

              ${packageName} = naersk.buildPackage {
                src = ./.;
                RUSTFLAGS = "-A missing_docs";
                FLAKE_REVISION = self.shortRev or
                  (builtins.replaceStrings [ "dirty" ] [ "modified" ]
                    self.dirtyShortRev);
              };
            };

          ######################################################################
          ##                            Devshells                             ##
          ######################################################################

          devshells =
            let
              git-z = inputs'.git-z.packages.git-z;

              dependencies = with pkgs; [
                sqlite
              ];

              buildToolchain = with pkgs; [
                rust-toolchain
                pkg-config
              ] ++ lib.optionals (!stdenv.isDarwin) [
                clang
              ];

              checkToolchain = with pkgs; [
                cargo-hack
                cargo-nextest
                committed
                eclint
                nixpkgs-fmt
                nodePackages.prettier
                taplo
                typos
              ];

              ideToolchain = with pkgs; [
                nixd
                rust-analyzer
              ];

              developmentTools = with pkgs; [
                cargo-bloat
                cargo-outdated
                cargo-watch
                diesel-cli
                git
                git-z
                gitAndTools.gitflow
              ];

              ideEnv = [
                {
                  name = "NIX_PATH";
                  value = "nixpkgs=${inputs.nixpkgs}";
                }
                {
                  name = "TYPOS_LSP_PATH";
                  value = "${pkgs.typos-lsp}/bin/typos-lsp";
                }
              ];
            in
            {
              default = {
                name = "gnucash-toolbox";

                motd = ''

                {202}🔨 Welcome to the gnucash-toolbox devshell!{reset}
              '';

                packages =
                  dependencies
                  ++ buildToolchain
                  ++ checkToolchain
                  ++ ideToolchain
                  ++ developmentTools;

                env =
                  ideEnv;

                commands = [
                  {
                    name = "build-deb";
                    command = "cargo deb --target=x86_64-unknown-linux-musl";
                  }

                  # Pass-through commands to make some cargo extensions run in
                  # their own devshell.
                  {
                    name = "cargo-deb";
                    command = "nix develop -L .#deb -c cargo $@";
                  }
                  {
                    name = "cargo-udeps";
                    command = "nix develop -L .#udeps -c cargo $@";
                  }
                ];
              };

              ci = {
                name = "gnucash-toolbox CI";

                packages =
                  dependencies
                  ++ buildToolchain
                  ++ checkToolchain;
              };

              # NOTE: Use the musl target to build a statically-linked binary.
              # We only add the target in a specialised devshell to avoid
              # cluttering the toolchain defined in `rust-toolchain.toml` on all
              # platforms.
              deb = {
                name = "cargo-deb";
                packages = with pkgs; [
                  (rust-toolchain.override {
                    targets = [ "x86_64-unknown-linux-musl" ];
                  })
                  clang
                  cargo-deb
                ];
              };

              # NOTE: cargo-udeps needs Rust nightly to run.
              udeps = {
                name = "cargo-udeps";
                packages = with pkgs; [
                  rust-bin.nightly."2024-10-28".minimal
                  clang
                  cargo-hack
                  cargo-udeps
                ];
              };
            };
        };
    };
}
