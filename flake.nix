{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.11";
    utils.url = "github:numtide/flake-utils";
    worker-build.url = "github:lemmih/nix-flakes?dir=worker-build";
    wrangler.url = "github:ryand56/wrangler/v4";
    rust-overlay.url = "github:oxalica/rust-overlay";
    alejandra.url = "github:kamadorueda/alejandra/3.1.0";
    crane.url = "github:ipetkov/crane";
    e2e = {
      url = "path:./e2e";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
    lychee.url = "github:lycheeverse/lychee";
  };

  outputs = {
    self,
    nixpkgs,
    utils,
    worker-build,
    wrangler,
    rust-overlay,
    alejandra,
    crane,
    e2e,
    advisory-db,
    lychee,
  }:
    utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [(import rust-overlay)];
        };
        pinned-wasm-bindgen-cli = pkgs.wasm-bindgen-cli.override {
          version = "0.2.100";
          hash = "sha256-3RJzK7mkYFrs7C/WkhW9Rr4LdP5ofb2FdYGz1P7Uxog=";
          cargoHash = "sha256-tD0OY2PounRqsRiFh8Js5nyknQ809ZcHMvCOLrvYHRE=";
        };
        worker-build-bin = worker-build.packages.${system}.default;
        wrangler-bin = wrangler.packages.${system}.default;
        lychee-bin = lychee.packages.${system}.default;

        rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

        # Initialize crane with our custom toolchain
        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

        # Common source filter for all Rust builds
        # This filters out files not needed for the Rust compilation
        src =
          craneLib.cleanCargoSource (craneLib.path ./.)
          // {
            # Add additional Rust source files that might not be included by default
            # For example, if you have files outside of src/ that are needed:
            # extraSrcGlobs = [
            #   ".cargo/config.toml"
            # ];
          };

        erudify-client-deps = craneLib.buildDepsOnly {
          inherit src;
          cargoExtraArgs = "--target wasm32-unknown-unknown --features hydrate --no-default-features";
          doCheck = false;
        };

        # Function to create client-side Wasm builds with configurable options
        makeErudifyClientBuild = {
          name,
          optimized ? true,
        }:
          craneLib.buildPackage {
            inherit src;
            cargoArtifacts = erudify-client-deps;
            buildPhaseCargoCommand = "HOME=$PWD/tmp wasm-pack build --out-dir pkg --mode no-install ${
              if optimized
              then "--release"
              else "--no-opt"
            } --no-typescript --target web --out-name client --features hydrate --no-default-features";
            doNotPostBuildInstallCargoBinaries = true;
            installPhaseCommand = ''
              mkdir -p $out/pkg
              cp -r pkg/* $out/pkg/
            '';
            doCheck = false;

            nativeBuildInputs = with pkgs;
              [
                wasm-pack
                pinned-wasm-bindgen-cli
              ]
              ++ (
                if optimized
                then [binaryen]
                else []
              );
          };

        # Create optimized and development client builds
        erudify-client = makeErudifyClientBuild {
          name = "erudify-client";
          optimized = true;
        };
        erudify-client-dev = makeErudifyClientBuild {
          name = "erudify-client-dev";
          optimized = false;
        };

        erudify-server-deps = craneLib.buildDepsOnly {
          inherit src;
          cargoExtraArgs = "--target wasm32-unknown-unknown --features ssr --no-default-features";
          doCheck = false;
        };

        # Function to create server-side Wasm builds with configurable options
        makeErudifyServerBuild = {
          name,
          optimized ? true,
        }:
          craneLib.buildPackage {
            inherit src;
            cargoArtifacts = erudify-server-deps;
            buildPhaseCargoCommand = "HOME=$PWD/tmp worker-build ${
              if optimized
              then "--release"
              else "--no-opt"
            } --features ssr --no-default-features";
            doNotPostBuildInstallCargoBinaries = true;
            doCheck = false;
            installPhaseCommand = ''
              mkdir -p $out/build
              cp -r build/* $out/build/
            '';

            nativeBuildInputs = with pkgs;
              [
                worker-build-bin
                pinned-wasm-bindgen-cli
                esbuild
              ]
              ++ (
                if optimized
                then [binaryen]
                else []
              );
          };

        # Create optimized and development server builds
        erudify-server = makeErudifyServerBuild {
          name = "erudify-server";
          optimized = true;
        };
        erudify-server-dev = makeErudifyServerBuild {
          name = "erudify-server-dev";
          optimized = false;
        };

        # For the main derivation, we need a different source set that includes non-Rust files
        mainSrc = pkgs.lib.cleanSourceWith {
          src = ./.;
          filter = path: type:
            (pkgs.lib.hasPrefix "${toString ./public}" path)
            || (pkgs.lib.hasPrefix "${toString ./style}" path)
            || (pkgs.lib.hasPrefix "${toString ./src}" path);
        };

        # Function to create a erudify derivation to reduce duplication
        makeErudifyDerivation = {
          name,
          clientBuild,
          serverBuild,
        }:
          pkgs.stdenv.mkDerivation {
            inherit name;
            src = mainSrc;

            nativeBuildInputs = with pkgs; [
              tailwindcss
            ];

            buildPhase = ''
              # Generate CSS
              tailwindcss --content "$src/**" -i ./style/tailwind.css -o style.css
            '';

            installPhase = ''
              # Create the output directory structure
              mkdir -p $out/assets

              # Copy static files
              cp -r $src/public/* $out/assets/

              # Copy generated CSS
              cp style.css $out/assets/style.css

              # Copy wasm build outputs from other derivations
              cp -r ${clientBuild}/* $out/assets/
              cp -r ${serverBuild}/build $out/
            '';
          };

        # Create production and development builds using the function
        erudify = makeErudifyDerivation {
          name = "erudify";
          clientBuild = erudify-client;
          serverBuild = erudify-server;
        };

        erudify-dev = makeErudifyDerivation {
          name = "erudify-dev";
          clientBuild = erudify-client-dev;
          serverBuild = erudify-server-dev;
        };

        # Create a function to setup wrangler environment
        makeWranglerScript = {
          name,
          wranglerArgs,
          verbose ? false,
        }:
          pkgs.writeScriptBin name ''
            #!${pkgs.bash}/bin/bash

            # Create a temporary directory for the environment
            WORK_DIR=$(mktemp -d)
            ${
              if verbose
              then "echo \"Created temporary directory: $WORK_DIR\""
              else ""
            }

            # Copy the wrangler configuration
            cp ${./wrangler.toml} $WORK_DIR/wrangler.toml
            ${
              if verbose
              then "echo \"Copied wrangler.toml to temporary directory\""
              else ""
            }

            # Setup the environment
            ln -s ${erudify} $WORK_DIR/result

            # Change to the work directory
            cd $WORK_DIR
            ${
              if verbose
              then "echo \"Changed to temporary directory\""
              else ""
            }

            # Run wrangler with the provided arguments
            ${
              if verbose
              then "echo \"Running wrangler with args: ${wranglerArgs}...\""
              else ""
            }
            exec ${wrangler-bin}/bin/wrangler ${wranglerArgs} "$@"
          '';

        # Create a development environment with a script to run wrangler
        erudify-preview = makeWranglerScript {
          name = "erudify-preview";
          wranglerArgs = "dev --env prebuilt --live-reload false";
        };

        # Create a deployment script for Cloudflare
        erudify-deploy = makeWranglerScript {
          name = "erudify-deploy";
          wranglerArgs = "deploy --env prebuilt";
          verbose = true;
        };

        e2e-test = pkgs.writeShellScriptBin "e2e-test" ''
          # Start the web service
          ${erudify-preview}/bin/erudify-preview &
          WEB_PID=$!

          ${pkgs.retry}/bin/retry --until=success --delay "1,2,4" -- curl -s http://localhost:8787/

          ${lychee-bin}/bin/lychee http://localhost:8787/
          LINK_CHECK_EXIT=$?
          if [ $LINK_CHECK_EXIT -ne 0 ]; then
            echo "Link check failed"
            kill $WEB_PID
            exit 1
          fi

          # Geckodriver is quite verbose, so we redirect the output to /dev/null
          # If you want to see the output, remove the redirection
          ${pkgs.geckodriver}/bin/geckodriver --port 4444 > /dev/null 2>&1 &
          GECKO_PID=$!

          # Run the tests
          ${self.packages.${system}.e2e}/bin/e2e
          TEST_EXIT=$?

          # Clean up
          kill $WEB_PID
          kill $GECKO_PID
          exit $TEST_EXIT
        '';

        # Clippy check for client code
        erudify-client-clippy = craneLib.cargoClippy {
          inherit src;
          cargoArtifacts = erudify-client-deps;
          cargoClippyExtraArgs = "--target wasm32-unknown-unknown --features hydrate --no-default-features -- --deny warnings";
        };

        # Clippy check for server code
        erudify-server-clippy = craneLib.cargoClippy {
          inherit src;
          cargoArtifacts = erudify-server-deps;
          cargoClippyExtraArgs = "--target wasm32-unknown-unknown --features ssr --no-default-features -- --deny warnings";
        };
      in {
        checks = {
          inherit erudify-client erudify-server;
          inherit erudify-client-clippy erudify-server-clippy;
          erudify-client-fmt = craneLib.cargoFmt {
            inherit src;
          };
          erudify-server-fmt = craneLib.cargoFmt {
            inherit src;
          };
          erudify-toml-fmt = craneLib.taploFmt {
            src = pkgs.lib.sources.sourceFilesBySuffices src [".toml"];
          };
          erudify-audit = craneLib.cargoAudit {
            inherit src advisory-db;
          };
          erudify-deny = craneLib.cargoDeny {
            inherit src;
          };
        };

        packages = {
          inherit erudify erudify-client erudify-server erudify-dev;
          e2e = e2e.packages.${system}.default;
          wrangler = wrangler-bin;
          default = erudify;
        };

        apps = rec {
          # Development app for local testing
          preview = {
            type = "app";
            program = "${erudify-preview}/bin/erudify-preview";
            meta.description = "Run ERUDIFY application in local development mode with wrangler";
          };

          default = preview;

          # Deployment app for Cloudflare
          deploy = {
            type = "app";
            program = "${erudify-deploy}/bin/erudify-deploy";
            meta.description = "Deploy ERUDIFY application to Cloudflare Workers";
          };

          # End-to-end test runner
          e2e = {
            type = "app";
            program = "${e2e-test}/bin/e2e-test";
            meta.description = "Run end-to-end tests - requires a local firefox";
          };
        };

        formatter = alejandra.packages.${system}.default;
      }
    );
}
