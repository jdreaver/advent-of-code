{
  inputs = {
    nixpkgs-unstable.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs-unstable }:
    let
      pkgs = import nixpkgs-unstable { system = "x86_64-linux"; config = { allowUnfree = true; }; };
    in {
      devShells.x86_64-linux.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          # Rust
          cargo
          rustc
          clippy
          rustfmt

          # C
          gcc
          gcc.man
          gdb
          valgrind
        ];
      };
    };
}
