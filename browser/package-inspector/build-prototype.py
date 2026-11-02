"""Build pinned Rust/WASM, independent original CLI controls and a prefixed site."""
import argparse,hashlib,json,os,re,shutil,subprocess,tempfile
from pathlib import Path
from controls import NAMES,requests

parser=argparse.ArgumentParser();parser.add_argument('--bindgen',required=True,type=Path);args=parser.parse_args()
B=Path(__file__).resolve().parent;R=B.parents[1];C=B/'.cache';C.mkdir(exist_ok=True)
env=os.environ.copy();target=Path(env.get('CARGO_TARGET_DIR',R/'target/cargo')).resolve();env['CARGO_TARGET_DIR']=str(target)
env['CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUSTFLAGS']='-C link-arg=--max-memory=67108864'
def run(argv,**kwargs):return subprocess.run([str(x) for x in argv],cwd=R,env=env,check=True,**kwargs)
assert subprocess.check_output([str(args.bindgen.resolve()),'--version']).decode().strip()=='wasm-bindgen 0.2.127'
run(['python',B/'verify-foundation.py'])
run(['cargo','+1.95.0','build','--locked','--bin','vtrace'])
for extra in [[],['--lib','--target','wasm32-unknown-unknown']]:
    run(['cargo','+1.95.0','build','--manifest-path',B/'Cargo.toml','--release','--locked',*extra])
S=R/'target/package-prototype/VTRACE';S.mkdir(parents=True,exist_ok=True)
shutil.copytree(B/'web',S,dirs_exist_ok=True)
run([args.bindgen.resolve(),target/'wasm32-unknown-unknown/release/vtrace_package_inspector.wasm','--target','web','--out-dir',S/'pkg'])
for file in (S/'pkg').glob('*.d.ts'):file.unlink()
suffix='.exe' if os.name=='nt' else ''
native=target/('release/vtrace-package-control'+suffix);original=target/('debug/vtrace'+suffix)
records=[];cli_controls=[]
with tempfile.TemporaryDirectory(prefix='vtrace independent cli ') as temporary:
    temporary=Path(temporary).resolve();directory=temporary/'docs/vtrace';directory.mkdir(parents=True)
    for request in requests(R,B):
        raw=request if isinstance(request,str) else json.dumps(request,ensure_ascii=False,separators=(',',':'))
        first=run([native],input=raw.encode(),capture_output=True).stdout
        assert first==run([native],input=raw.encode(),capture_output=True).stdout
        response=json.loads(first);records.append({'request':raw,'response':response})
        if response['kind']!='report':continue
        source=json.loads(raw)
        for name in NAMES:
            path=directory/name;assert path.parent==directory
            if path.exists():path.unlink()
        for document in source['files']:(directory/document['name']).write_bytes(document['text'].encode('utf-8'))
        result=subprocess.run([str(original),'validate',str(temporary)],cwd=R,env=env,capture_output=True)
        assert result.returncode in [0,1],result.stderr
        findings=[]
        # Rust println separates records with LF. Lone CR/U+2028 may be data.
        for line in result.stdout.decode('utf-8').split('\n'):
            match=re.fullmatch(r'(ERROR|WARNING): (.+):(\d+): (.*)',line)
            if match:findings.append({'level':match[1],'path':match[2].replace('\\','/'),'line':int(match[3]),'message':match[4]})
        assert findings==response['findings'],(len(records)-1,findings,response['findings'])
        cli_controls.append({'request_index':len(records)-1,'exit_code':result.returncode,'findings':findings,'stdout_sha256':hashlib.sha256(result.stdout).hexdigest()})
assert len(records)==36 and len(cli_controls)==17
assert len(records[-2]['response']['findings'])==1 and not records[-1]['response']['findings']
assert records[-1]['response']['summary']['open_work_packages']==['WP-001']
assert set(document['name'] for document in json.loads((B/'web/example.json').read_text(encoding='utf-8'))['files'])==set(records[0]['response']['accepted_artifact_names'])
(C/'native-cases.json').write_text(json.dumps(records,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
(C/'cli-controls.json').write_text(json.dumps(cli_controls,indent=2)+'\n',encoding='utf-8')
(C/'native-controls.json').write_text(json.dumps({'native_cases':len(records),'each_native_repeated':2,'independent_original_cli_findings_controls':len(cli_controls),'cli_path_normalization':'Display separators only; original findings/messages/line/order unchanged.'},indent=2)+'\n',encoding='utf-8')
print(json.dumps({'site':str(S),'native_cases':len(records),'original_cli_controls':len(cli_controls),'raw_wasm_bytes':(S/'pkg/vtrace_package_inspector_bg.wasm').stat().st_size}))
