with import <nixpkgs> { };
mkShell {
  buildInputs = [
    gcc
    libGLU
    freeglut
  ];
}
