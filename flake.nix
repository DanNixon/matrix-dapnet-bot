{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    naersk.url = "github:nix-community/naersk";
  };

  outputs = { self, nixpkgs, flake-utils, fenix, naersk }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = (import nixpkgs) {
          inherit system;
        };

        toolchain = fenix.packages.${system}.toolchainOf {
          channel = "1.72";
          sha256 = "Q9UgzzvxLi4x9aWUJTn+/5EXekC98ODRU1TwhUs9RnY=";
        };

        naersk' = pkgs.callPackage naersk {
          cargo = toolchain.rust;
          rustc = toolchain.rust;
        };

        nativeBuildInputs = with pkgs; [ cmake pkg-config ];
        buildInputs = with pkgs; [ openssl ];

      in rec {
        devShell = pkgs.mkShell {
          nativeBuildInputs = nativeBuildInputs ++ [ toolchain.toolchain ];
          buildInputs = buildInputs;
          packages = with pkgs; [ skopeo ];
        };

        packages = {
          default = naersk'.buildPackage {
            src = ./.;
            nativeBuildInputs = nativeBuildInputs;
            buildInputs = buildInputs;
          };

          container-image = pkgs.dockerTools.buildImage {
            name = "matrix-dapnet-bot";
            tag = "latest";
            created = "now";

            copyToRoot = pkgs.buildEnv {
              name = "image-root";
              paths = [ pkgs.tini packages.default ];
              pathsToLink = [ "/bin" ];
            };

            config = {
              Entrypoint = [ "/bin/tini" "--" "/bin/matrix-dapnet-bot" ];
              ExposedPorts = {
                "9090/tcp" = {};
              };
              Env = [
                "OBSERVABILITY_ADDRESS=0.0.0.0:9090"
              ];
            };
          };

          fmt = naersk'.buildPackage {
            src = ./.;
            nativeBuildInputs = nativeBuildInputs;
            buildInputs = buildInputs;
            mode = "fmt";
          };

          clippy = naersk'.buildPackage {
            src = ./.;
            nativeBuildInputs = nativeBuildInputs;
            buildInputs = buildInputs;
            mode = "clippy";
          };

          test = naersk'.buildPackage {
            src = ./.;
            nativeBuildInputs = nativeBuildInputs;
            buildInputs = buildInputs;
            mode = "test";
            cargoTestOptions = x: x ++ ["1>&2"];
          };
        };
      }
    );
}
