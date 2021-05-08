source $stdenv/setup

mkdir -p $out/bin
echo "echo hello" > $out/bin/hello
chmod +x $out/bin/hello
