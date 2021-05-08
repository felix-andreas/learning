import sys
from pathlib import Path
import re

if len(sys.argv) > 1:
    sources = [Path(arg) for arg in sys.argv[1:]]
else:
    sources = list(Path.cwd().rglob("*.dmp"))

pattern = re.compile("#.*?\n")

for source in sources:
    text = source.read_text()
    no_comments = re.sub(pattern, "", text)
    no_whitespace = "".join(no_comments.split())
    print(no_whitespace)

    build_dir = source.parent / "build"
    build_dir.mkdir(exist_ok=True)
    executable = build_dir / source.stem
    executable.write_bytes(bytes.fromhex(no_whitespace))
    executable.chmod(executable.stat().st_mode | 0o111)
