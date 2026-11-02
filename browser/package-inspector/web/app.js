const $=id=>document.getElementById(id);
const encoder=new TextEncoder();
let draft=new Map(),originals=new Map(),selected='',revision=0,readJob=0,activeRead=0;
let accepted=null,allowedNames=[],worker,epoch=0,job=0,pending=null,ready=false,startupTimer;
const request=()=>JSON.stringify({files:[...draft].map(([name,text])=>({name,text}))});
const status=text=>{$('status').textContent=text;};
function custody(){
  $('draft-status').textContent=accepted&&request()!==accepted.request?'Draft changed. Findings and accepted exports still describe the earlier accepted input.':accepted?'Draft matches the accepted input snapshot. Findings are structural observations only.':'The synthetic example has a missing requirement reference: replace REQ-002 with REQ-001 in TRACE.md, then check again.';
}
function controls(){
  $('run').disabled=!ready||!!pending;
  $('accepted-input').disabled=$('accepted-report').disabled=!accepted;
  $('editor').disabled=!draft.has(selected);
  $('remove').disabled=$('current-file').disabled=$('lf-file').disabled=!draft.has(selected);
  $('add').disabled=!$('new-artifact').options.length;
  $('files').disabled=!allowedNames.length;
  $('original-file').disabled=!originals.has($('original-list').value);
  custody();
}
function invalidate(){if(activeRead){$('read-status').textContent='Input changed. Pending read superseded; previous working inputs retained.';activeRead=0;}revision++;readJob++;}
function selectFile(name){
  selected=draft.has(name)?name:'';$('file-list').value=selected;
  $('editor').value=selected?draft.get(selected):'';
  $('editor-label').textContent=selected?`Edit ${selected}`:'File text';
  $('editor-status').textContent=selected?`${encoder.encode(draft.get(selected)).length} input UTF-8 bytes. Browser editing normalizes line endings; original bytes remain separate.`:'Choose or add a file to edit.';
  controls();
}
function refreshDraft(preferred=selected){
  const originalSelected=$('original-list').value;
  $('file-list').replaceChildren(...[...draft.keys()].map(name=>new Option(name,name)));
  $('original-list').replaceChildren(...[...originals.keys()].map(name=>new Option(name,name)));
  if(originals.has(originalSelected))$('original-list').value=originalSelected;
  const available=allowedNames.filter(name=>!draft.has(name));
  $('new-artifact').replaceChildren(...available.map(name=>new Option(name,name)));
  selectFile(draft.has(preferred)?preferred:[...draft.keys()][0]??'');
}
function fail(message){
  clearTimeout(startupTimer);if(pending)clearTimeout(pending.timer);
  pending=null;ready=false;epoch++;worker?.terminate();
  status(`${message} Accepted report and input exports retained. Use Retry engine.`);controls();
}
function start(){
  worker?.terminate();clearTimeout(startupTimer);if(pending)clearTimeout(pending.timer);
  pending=null;ready=false;const generation=++epoch;status('Loading Rust engine…');controls();
  try {worker=new Worker(new URL('./worker.js',import.meta.url),{type:'module'});}catch(error){fail(String(error));return;}
  startupTimer=setTimeout(()=>{if(generation===epoch)fail('Engine startup exceeded 12 seconds.');},12000);
  worker.onerror=event=>{if(generation===epoch){event.preventDefault();fail(`Worker error: ${event.message}`);}};
  worker.onmessage=({data})=>{
    if(generation!==epoch)return;
    if(data.type==='ready'){
      clearTimeout(startupTimer);ready=true;allowedNames=[...data.artifactNames];refreshDraft();
      status(accepted?'Rust ready. Accepted input/report retained.':'Rust ready. Open files or load the example, then check the package.');
      controls();if(!accepted&&draft.size)run();return;
    }
    if(data.type==='failure'){if(data.id===undefined||data.id===pending?.id)fail(`Engine failure: ${data.message}`);return;}
    if(data.type!=='result'||!pending||data.id!==pending.id)return;
    const operation=pending;clearTimeout(operation.timer);pending=null;
    if(operation.revision!==revision){status('Input changed during computation. Stale result discarded; accepted report retained.');controls();return;}
    if(data.report.kind!=='report'){status(`${data.report.kind}: ${data.report.message}. Accepted input/report retained.`);controls();return;}
    accepted={request:operation.request,report:data.report};allowedNames=[...data.report.accepted_artifact_names];
    renderReport();refreshDraft();status('Rust report accepted. Structural checks only; evidence and reviews remain caller assertions.');controls();
  };
}
function run(){
  if(!ready||pending)return;
  const raw=request();const id=++job;
  pending={id,request:raw,revision,timer:setTimeout(()=>fail('Computation exceeded 10 seconds.'),10000)};
  status('Rust is checking the supplied package…');controls();worker.postMessage({id,request:raw});
}
function renderReport(){
  const r=accepted.report;
  $('result-summary').textContent=r.findings.length?`${r.findings.length} structural finding${r.findings.length===1?'':'s'} in the accepted input.`:'No structural findings from these native checks. This is not a readiness certificate.';
  const counts=[['Requirements',r.summary.requirements],['Specifications',r.summary.specs],['Evidence IDs',r.summary.evidence_rows],['Work packages',r.summary.work_packages],['Open work packages (caller status)',r.summary.open_work_packages.length]];
  $('counts').replaceChildren(...counts.map(([label,value])=>{const n=document.createElement('span');n.className='count';n.textContent=`${label}: ${value}`;return n;}));
  $('findings').replaceChildren(...r.findings.map(finding=>{
    const item=document.createElement('li');const button=document.createElement('button');
    button.type='button';button.textContent=`${finding.path}:${finding.line}`;button.onclick=()=>showSource(finding);
    const text=document.createElement('p');text.textContent=`${finding.level}: ${finding.message}`;item.append(button,text);return item;
  }));
  $('work-rows').textContent=JSON.stringify(r.work_packages,null,2);$('review-rows').textContent=JSON.stringify(r.review_lanes,null,2);
  $('comms-rows').textContent=JSON.stringify(r.communication_surfaces,null,2);$('report-json').textContent=JSON.stringify(r,null,2);
  $('source-title').textContent='Accepted source at the finding';$('source-note').textContent='';$('source-lines').replaceChildren();$('source-view').open=false;
}
function showSource(finding){
  const name=finding.path.replace(/^docs\/vtrace\//,'');
  const file=JSON.parse(accepted.request).files.find(file=>file.name===name);
  $('source-title').textContent=`Accepted ${name} · native line ${finding.line}`;$('source-view').open=true;
  if(!file){$('source-lines').replaceChildren();$('source-note').textContent='This artifact was not supplied in the accepted input. Add it to the draft and check again.';return;}
  const lines=file.text.split('\n');if(lines.at(-1)==='')lines.pop();
  $('source-lines').replaceChildren(...lines.map((line,index)=>{const li=document.createElement('li');li.textContent=line.replace(/\r$/,'');if(index+1===finding.line)li.className='highlight';return li;}));
  const same=draft.get(name)===file.text;
  $('source-note').textContent=same?'Native line view. Original line-ending bytes remain in accepted/original exports.':'The draft differs. This view belongs to the accepted input; your current edits are retained.';
  if(same){
    selectFile(name);
    let start=0;for(let line=1;line<finding.line;line++){const next=file.text.indexOf('\n',start);start=next<0?file.text.length:next+1;}
    const end=file.text.indexOf('\n',start);const normalize=text=>text.replace(/\r\n?/g,'\n');
    $('editor').focus();$('editor').setSelectionRange(normalize(file.text.slice(0,start)).length,normalize(file.text.slice(0,end<0?file.text.length:end)).length);
  }
  $('source-lines').querySelector('.highlight')?.scrollIntoView({block:'nearest'});
}
function download(name,body,type='text/plain;charset=utf-8'){
  const blob=body instanceof Blob?body:new Blob([body],{type});const url=URL.createObjectURL(blob);const link=document.createElement('a');link.href=url;link.download=name;link.click();setTimeout(()=>URL.revokeObjectURL(url),30000);
}
async function loadExample(){
  invalidate();const token=readJob;const currentRevision=revision;const controller=new AbortController();
  activeRead=token;
  const timer=setTimeout(()=>controller.abort(),5000);$('read-status').textContent='Loading synthetic example…';
  try{
    const response=await fetch('./example.json',{signal:controller.signal});if(!response.ok)throw Error(`Example HTTP ${response.status}`);
    const example=await response.json();if(token!==readJob||currentRevision!==revision)return;
    draft=new Map(example.files.map(file=>[file.name,file.text]));revision++;refreshDraft('TRACE.md');
    $('read-status').textContent='Synthetic example loaded. Its listed evidence and review statuses are fixture assertions, not executed evidence.';
    if(ready&&!pending)run();
  }catch(error){if(token===readJob)$('read-status').textContent=`Example load failed: ${error.message}. Current draft and accepted input/report retained.`;}
  finally{clearTimeout(timer);if(activeRead===token)activeRead=0;}
}
$('files').onchange=async()=>{
  const files=[...$('files').files];if(!files.length)return;
  invalidate();const token=readJob;const currentRevision=revision;let timer;$('read-status').textContent='Reading selected files…';
  activeRead=token;
  try{
    if(files.length>11)throw Error('At most eleven files per opening');
    const seen=new Set();for(const file of files){if(!allowedNames.includes(file.name))throw Error('Choose an exact supported docs/vtrace artifact filename');if(seen.has(file.name))throw Error('Duplicate filename in this opening');seen.add(file.name);if(file.size>65536)throw Error(`${file.name} exceeds 64 KiB`);}
    const currentBytes=[...draft].filter(([name])=>!seen.has(name)).reduce((sum,[,text])=>sum+encoder.encode(text).length,0);
    if(currentBytes+files.reduce((sum,file)=>sum+file.size,0)>262144)throw Error('Complete draft exceeds 256 KiB');
    const opened=await Promise.race([Promise.all(files.map(async file=>{const bytes=await file.arrayBuffer();if(bytes.byteLength>65536)throw Error(`${file.name} exceeds 64 KiB`);return {name:file.name,bytes,text:new TextDecoder('utf-8',{fatal:true,ignoreBOM:true}).decode(bytes)};})),new Promise((_,reject)=>{timer=setTimeout(()=>reject(Error('File read exceeded 5 seconds')),5000);})]);
    if(token!==readJob||currentRevision!==revision)return;
    const next=new Map(draft),raw=new Map(originals);
    for(const file of opened){next.set(file.name,file.text);raw.set(file.name,new Blob([file.bytes]));}
    draft=next;originals=raw;revision++;refreshDraft(opened[0].name);
    $('read-status').textContent=`Opened ${opened.length} UTF-8 file${opened.length===1?'':'s'}. Original bytes retained; check the draft explicitly.`;
  }catch(error){if(token===readJob)$('read-status').textContent=`${error.message}. Previous draft, originals and accepted input/report retained.`;}
  finally{clearTimeout(timer);if(activeRead===token)activeRead=0;if(token===readJob)$('files').value='';}
};
$('editor').oninput=()=>{if(!draft.has(selected))return;invalidate();draft.set(selected,$('editor').value);$('editor-status').textContent=`${encoder.encode($('editor').value).length} draft UTF-8 bytes. Check again to update the accepted report.`;controls();};
$('editor').onkeydown=event=>{if(event.key==='Enter'&&(event.ctrlKey||event.metaKey)){event.preventDefault();run();}};
$('file-list').onchange=()=>selectFile($('file-list').value);
$('original-list').onchange=controls;
$('add').onclick=()=>{const name=$('new-artifact').value;if(!name)return;invalidate();draft.set(name,'');refreshDraft(name);};
$('remove').onclick=()=>{invalidate();draft.delete(selected);refreshDraft();};
$('clear').onclick=()=>{invalidate();draft.clear();refreshDraft();};
$('current-file').onclick=()=>{if(draft.has(selected))download(selected,draft.get(selected));};
$('lf-file').onclick=()=>{if(draft.has(selected))download(selected.replace(/\.md$/,'')+'-editor-lf.md',$('editor').value.replace(/\r\n?/g,'\n'));};
$('draft-package').onclick=()=>download('vtrace-current-input.json',request(),'application/json');
$('original-file').onclick=()=>{const name=$('original-list').value;if(originals.has(name))download(name,originals.get(name));};
$('accepted-input').onclick=()=>{if(accepted)download('vtrace-accepted-input.json',accepted.request,'application/json');};
$('accepted-report').onclick=()=>{if(accepted)download('vtrace-accepted-report.json',JSON.stringify(accepted.report,null,2),'application/json');};
$('run').onclick=run;$('retry').onclick=start;$('example').onclick=loadExample;
controls();start();loadExample();
