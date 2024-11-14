{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    llvmPackages_18.libllvm
    libffi
    libxml2
    clang
  ];
  LLVM_SYS_180_PREFIX = "${pkgs.llvmPackages_18.libllvm.dev}";
}