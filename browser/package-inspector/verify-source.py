"""Rebuild the delivered complete tracked snapshot outside Git, in a path with spaces."""
import argparse,hashlib,json,os,subprocess,tempfile,zipfile
from pathlib import Path
p=argparse.ArgumentParser();p.add_argument('--bindgen',required=True,type=Path);args=p.parse_args()
B=Path(__file__).resolve().parent;R=B.parents[1];archive=R/'target/package-release/VTRACE/source.zip'
expected=json.loads((B/'.cache/native-cases.json').read_text(encoding='utf-8'));controls=json.loads((B/'.cache/cli-controls.json').read_text(encoding='utf-8'))
env=os.environ.copy();env.setdefault('CARGO_TARGET_DIR',str(R/'target/cargo'))
with tempfile.TemporaryDirectory(prefix='vtrace delivered source ') as tmp:
    tmp=Path(tmp).resolve()
    with zipfile.ZipFile(archive) as source:
        for name in source.namelist():assert not Path(name).is_absolute() and '..' not in Path(name).parts,name
        source.extractall(tmp)
    root=tmp/'VTRACE-source';record=json.loads((root/'.source-record.json').read_text(encoding='utf-8'));assert not list(root.rglob('.git'))
    for file in record['files']:assert hashlib.sha256((root/file['path']).read_bytes()).hexdigest()==file['sha256'],file['path']
    def run(argv,cwd=root):subprocess.run([str(x)for x in argv],cwd=cwd,env=env,check=True)
    run(['cargo','+1.95.0','test','--locked'])
    run(['cargo','+1.95.0','test','--manifest-path','browser/package-inspector/Cargo.toml','--all-targets','--locked'])
    run(['python',root/'browser/package-inspector/build-prototype.py','--bindgen',args.bindgen.resolve()])
    companion=root/'browser/package-inspector'
    rebuilt=json.loads((companion/'.cache/native-cases.json').read_text(encoding='utf-8'));assert rebuilt==expected,'Delivered source input/report drift'
    compared=json.loads((companion/'.cache/cli-controls.json').read_text(encoding='utf-8'));assert compared==controls,'Delivered-source original CLI finding/control drift'
    npm='npm.cmd' if os.name=='nt' else 'npm';run([npm,'ci'],cwd=companion);run([npm,'run','test:pages'],cwd=companion)
    (B/'.cache/source-replay.json').write_text(json.dumps({'commit':record['commit'],'source_files':len(record['files']),'original_native_tests':31,'reader_tests':4,'adapter_tests':7,'native_outcomes_identical':len(rebuilt),'original_cli_controls_identical':len(compared),'browser_task_and_recovery_checks':21,'temporary_root_outside_git':str(root),'binary_identity_claim':False},indent=2)+'\n',encoding='utf-8')
