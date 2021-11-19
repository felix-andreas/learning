with import <nixpkgs> {};
derivation {
  name = "foo";
  builder = "${bash}/bin/bash";
  args = [ ./builder-foo.sh ];
  system = builtins.currentSystem;
}
