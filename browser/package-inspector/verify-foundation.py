"""Check canonical Git text identity of original native entrypoints and tests."""
import hashlib,json
from pathlib import Path
B=Path(__file__).resolve().parent;R=B.parents[1]
record=json.loads((B/'native-foundation.json').read_text(encoding='utf-8'))
for path,expected in record['git_lf_files_sha256'].items():
    assert hashlib.sha256((R/path).read_bytes().replace(b'\r\n',b'\n')).hexdigest()==expected,path
unit=(R/'src/lib.rs').read_bytes().replace(b'\r\n',b'\n').split(b'#[cfg(test)]',1)[1]
assert hashlib.sha256(unit).hexdigest()==record['original_unit_module_sha256'],'Original nine unit tests changed'
print('Original native CLI,31tests,manifest and lock retain their canonical Git text.')
