import {test,expect} from '@playwright/test';
import fs from 'node:fs';
const cases=JSON.parse(fs.readFileSync(new URL('../.cache/native-cases.json',import.meta.url),'utf8'));
async function open(page){await page.goto('./');await expect(page.locator('#status')).toContainText('Rust report accepted');}
async function check(page){await page.locator('#run').click();await expect(page.locator('#status')).toContainText('Rust report accepted');}
async function originals(page){const section=page.locator('details:has(#original-list)');if(await section.getAttribute('open')===null)await section.locator('summary').click();}
async function download(page,id){if(id==='original-file')await originals(page);const event=page.waitForEvent('download');await page.locator(`#${id}`).click();return fs.readFileSync(await(await event).path());}
async function input(page){return JSON.parse((await download(page,'accepted-input')).toString());}

test('synthetic missing trace resolves through an own edit without converting caller status to proof',async({page})=>{
  await open(page);await expect(page.locator('#findings li')).toHaveCount(1);
  await page.locator('#file-list').selectOption('TRACE.md');const original=await page.locator('#editor').inputValue();await page.locator('#editor').fill(original.replace('REQ-002','REQ-001'));await check(page);
  await expect(page.locator('#findings li')).toHaveCount(0);await expect(page.locator('#result-summary')).toContainText('not a readiness certificate');
  await expect(page.locator('#counts')).toContainText('Open work packages (caller status): 1');
  const report=JSON.parse((await download(page,'accepted-report')).toString());expect(report.summary.requirements).toBe(1);expect(report.summary.open_work_packages).toEqual(['WP-001']);expect(report.review_lanes[0].decision).toBe('passed');expect(report.scope).toContain('caller assertions');
});

test('all native outcomes match the real module worker and the compiled memory maximum is exactly 64 MiB',async({page})=>{
  await open(page);
  const result=await page.evaluate(async records=>{
    const worker=new Worker(new URL('./worker.js',location.href),{type:'module'});
    await new Promise((resolve,reject)=>{worker.onerror=reject;worker.onmessage=({data})=>data.type==='ready'?resolve():reject(data);});
    const mismatches=[];
    for(let i=0;i<records.length;i++){
      const actual=await new Promise((resolve,reject)=>{worker.onmessage=({data})=>data.type==='result'&&data.id===i?resolve(data.report):reject(data);worker.postMessage({id:i,request:records[i].request});});
      if(JSON.stringify(actual)!==JSON.stringify(records[i].response))mismatches.push(i);
    }
    worker.terminate();
    const {default:init}=await import('./pkg/vtrace_package_inspector.js');const module=await init();
    module.memory.grow(1024-module.memory.buffer.byteLength/65536);const maximum=module.memory.buffer.byteLength;
    let refused=false;try{module.memory.grow(1);}catch{refused=true;}
    return {cases:records.length,mismatches,maximum,refused};
  },cases);
  expect(result).toEqual({cases:35,mismatches:[],maximum:67108864,refused:true});
});

test('finding navigation shows accepted source and preserves a newer draft',async({page})=>{
  await open(page);const finding=page.getByRole('button',{name:'docs/vtrace/REQUIREMENTS.md:3',exact:true});await finding.click();
  await expect(page.locator('#file-list')).toHaveValue('REQUIREMENTS.md');
  expect(await page.locator('#editor').evaluate(editor=>editor.value.slice(editor.selectionStart,editor.selectionEnd))).toContain('REQ-001');
  await expect(page.locator('#source-lines li.highlight')).toContainText('REQ-001');
  const changed='# inserted draft line\n'+await page.locator('#editor').inputValue();await page.locator('#editor').fill(changed);await finding.click();
  await expect(page.locator('#editor')).toHaveValue(changed);await expect(page.locator('#source-note')).toContainText('draft differs');await expect(page.locator('#source-lines')).not.toContainText('inserted draft line');
  const draft=JSON.parse((await download(page,'draft-package')).toString());expect(draft.files.find(file=>file.name==='REQUIREMENTS.md').text).toBe(changed);
});

test('BOM and CRLF originals, current input and editor LF exports have separate byte custody',async({page})=>{
  await open(page);const bytes=Buffer.from('\ufeff| ID | Text |\r\n|---|---|\r\n| REQ-001 | owned |\r\n');
  await page.locator('#files').setInputFiles({name:'REQUIREMENTS.md',mimeType:'text/markdown',buffer:bytes});await expect(page.locator('#read-status')).toContainText('Opened 1');
  expect(await download(page,'original-file')).toEqual(bytes);expect(await download(page,'current-file')).toEqual(bytes);
  expect((await download(page,'lf-file')).toString()).toBe(await page.locator('#editor').inputValue());
  await check(page);expect((await input(page)).files.find(file=>file.name==='REQUIREMENTS.md').text).toBe(bytes.toString());
  const accepted=await download(page,'accepted-input');await page.locator('#editor').fill('edited');expect(await download(page,'original-file')).toEqual(bytes);expect(await download(page,'accepted-input')).toEqual(accepted);expect((await download(page,'current-file')).toString()).toBe('edited');
});

test('a failed multi-file opening commits neither a partial draft nor original byte collection',async({page})=>{
  await open(page);const before=await download(page,'draft-package');const accepted=await download(page,'accepted-input');
  await page.locator('#files').setInputFiles([{name:'MISSION.md',mimeType:'text/plain',buffer:Buffer.from('new')},{name:'TRACE.md',mimeType:'text/plain',buffer:Buffer.from([0xff])}]);
  await expect(page.locator('#read-status')).toContainText('Previous draft, originals');expect(await download(page,'draft-package')).toEqual(before);expect(await download(page,'accepted-input')).toEqual(accepted);await expect(page.locator('#original-file')).toBeDisabled();
});

test('oversized, unknown and duplicate opening names are refused without replacing working inputs',async({page})=>{
  await open(page);const before=await download(page,'draft-package');
  for(const files of [[{name:'MISSION.md',mimeType:'text/plain',buffer:Buffer.alloc(65537,120)}],[{name:'../other.md',mimeType:'text/plain',buffer:Buffer.from('unknown')}],[{name:'TRACE.md',mimeType:'text/plain',buffer:Buffer.from('one')},{name:'TRACE.md',mimeType:'text/plain',buffer:Buffer.from('two')}]]){
    await page.locator('#files').setInputFiles(files);await expect(page.locator('#read-status')).toContainText('Previous draft, originals');expect(await download(page,'draft-package')).toEqual(before);
  }
});

test('an empty logical package and added empty artifact preserve native presence semantics',async({page})=>{
  await open(page);await page.locator('#clear').click();await check(page);await expect(page.locator('#findings li')).toHaveCount(6);
  await page.getByRole('button',{name:'docs/vtrace/MISSION.md:1',exact:true}).click();await expect(page.locator('#source-note')).toContainText('was not supplied');
  await page.locator('#new-artifact').selectOption('MISSION.md');await page.locator('#add').click();await check(page);await expect(page.locator('#findings li')).toHaveCount(5);expect((await input(page)).files).toEqual([{name:'MISSION.md',text:''}]);
});

test('unknown Markdown columns and untrusted HTML remain original input and inert native text',async({page})=>{
  await open(page);const text='| ID | Objective | Status | Extra |\n|---|---|---|---|\n| WP-001 | <img src=x onerror="window.injected=1"> | passed | own extension |\n';
  await page.locator('#files').setInputFiles({name:'WORK_PACKAGES.md',mimeType:'text/plain',buffer:Buffer.from(text)});await expect(page.locator('#read-status')).toContainText('Opened 1');await check(page);
  expect((await input(page)).files.find(file=>file.name==='WORK_PACKAGES.md').text).toBe(text);const report=JSON.parse((await download(page,'accepted-report')).toString());expect(report.work_packages[0].objective).toContain('<img');expect(report.work_packages[0].Extra).toBeUndefined();expect(await page.locator('img').count()).toBe(0);expect(await page.evaluate(()=>window.injected)).toBeUndefined();
});

test('Rust line refusal retains complete draft and accepted exports',async({page})=>{
  await open(page);const before=await download(page,'accepted-input');await page.locator('#file-list').selectOption('MISSION.md');await page.locator('#editor').fill('x'.repeat(2049));await page.locator('#run').click();await expect(page.locator('#status')).toContainText('browser_refusal');
  await expect(page.locator('#editor')).toHaveValue('x'.repeat(2049));expect(await download(page,'accepted-input')).toEqual(before);await page.locator('#editor').fill('repaired');await check(page);expect((await input(page)).files.find(file=>file.name==='MISSION.md').text).toBe('repaired');
});

test('native line locations map to browser caret offsets with Unicode and lone CR',async({page})=>{
  await open(page);const text='# 👩‍💻\rone native line\n| ID | Text |\n|---|---|\n| REQ-001 | owned |\n';
  await page.locator('#files').setInputFiles({name:'REQUIREMENTS.md',mimeType:'text/plain',buffer:Buffer.from(text)});await expect(page.locator('#read-status')).toContainText('Opened 1');await check(page);await page.getByRole('button',{name:'docs/vtrace/REQUIREMENTS.md:4',exact:true}).click();
  expect(await page.locator('#editor').evaluate(editor=>editor.value.slice(editor.selectionStart,editor.selectionEnd))).toContain('REQ-001');expect((await input(page)).files.find(file=>file.name==='REQUIREMENTS.md').text).toBe(text);
});

test('keyboard check, retry and removed-file originals retain their own state',async({page})=>{
  await open(page);await page.locator('#file-list').selectOption('TRACE.md');await page.locator('#editor').fill((await page.locator('#editor').inputValue()).replace('REQ-002','REQ-001'));await page.locator('#editor').press('Control+Enter');await expect(page.locator('#findings li')).toHaveCount(0);
  const before=await download(page,'accepted-input');await page.locator('#retry').click();await expect(page.locator('#status')).toContainText('Accepted input/report retained');expect(await download(page,'accepted-input')).toEqual(before);
  const bytes=Buffer.from('opened original');await page.locator('#files').setInputFiles({name:'MISSION.md',mimeType:'text/plain',buffer:bytes});await expect(page.locator('#read-status')).toContainText('Opened 1');await page.locator('#remove').click();await originals(page);await page.locator('#original-list').selectOption('MISSION.md');expect(await download(page,'original-file')).toEqual(bytes);
});

test('mobile layout fits and export actions remain full width',async({page})=>{
  await page.setViewportSize({width:390,height:844});await open(page);expect(await page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth)).toBe(true);
  for(const id of ['draft-package','accepted-input','accepted-report'])expect((await page.locator(`#${id}`).boundingBox()).width).toBeGreaterThan(250);
});
