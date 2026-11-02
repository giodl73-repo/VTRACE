import {test,expect} from '@playwright/test';
import fs from 'node:fs';
async function open(page){await page.goto('./');await expect(page.locator('#status')).toContainText('Rust report accepted');}
async function accepted(page){const event=page.waitForEvent('download');await page.locator('#accepted-input').click();return fs.readFileSync(await(await event).path());}
async function draft(page){const event=page.waitForEvent('download');await page.locator('#draft-package').click();return JSON.parse(fs.readFileSync(await(await event).path(),'utf8'));}
const init=`import init,{evaluate} from './pkg/vtrace_package_inspector.js';await init();postMessage({type:'ready',artifactNames:JSON.parse(evaluate('{"files":[]}')).accepted_artifact_names});`;
const delayed=init+`self.onmessage=({data})=>setTimeout(()=>postMessage({type:'result',id:data.id,report:JSON.parse(evaluate(data.request))}),600);`;

test('edits made during computation fence the completed older result',async({page})=>{
  await page.route('**/worker.js',route=>route.fulfill({contentType:'text/javascript',body:delayed}));await open(page);const before=await accepted(page);
  await page.locator('#run').click();await page.locator('#editor').fill('newer draft while running');await expect(page.locator('#status')).toContainText('Stale result discarded');expect(await accepted(page)).toEqual(before);await expect(page.locator('#editor')).toHaveValue('newer draft while running');
});

test('retry fences the old worker and retains both accepted and current inputs',async({page})=>{
  await page.route('**/worker.js',route=>route.fulfill({contentType:'text/javascript',body:delayed}));await open(page);const before=await accepted(page);await page.locator('#editor').fill('new draft');await page.locator('#run').click();await page.locator('#retry').click();
  await expect(page.locator('#status')).toContainText('Accepted input/report retained');await page.waitForTimeout(800);expect(await accepted(page)).toEqual(before);await expect(page.locator('#editor')).toHaveValue('new draft');
  await page.locator('#run').click();await expect(page.locator('#status')).toContainText('Rust report accepted');expect(await accepted(page)).not.toEqual(before);
});

test('real ten-second compute deadline preserves exports and retry recovers',async({page})=>{
  await page.route('**/worker.js',route=>route.fulfill({contentType:'text/javascript',body:init+`let count=0;self.onmessage=({data})=>{if(count++)return;postMessage({type:'result',id:data.id,report:JSON.parse(evaluate(data.request))});};`}));await open(page);const before=await accepted(page);await page.locator('#editor').fill('candidate retained');await page.locator('#run').click();
  await expect(page.locator('#status')).toContainText('Computation exceeded 10 seconds',{timeout:12000});expect(await accepted(page)).toEqual(before);await expect(page.locator('#editor')).toHaveValue('candidate retained');
  await page.unroute('**/worker.js');await page.locator('#retry').click();await expect(page.locator('#run')).toBeEnabled();await page.locator('#run').click();await expect(page.locator('#status')).toContainText('Rust report accepted');
});

test('real twelve-second startup deadline leaves source routes available and retry works',async({page})=>{
  await page.route('**/worker.js',route=>route.fulfill({contentType:'text/javascript',body:'/* no ready event */'}));await page.goto('./');await expect(page.locator('#status')).toContainText('Engine startup exceeded 12 seconds',{timeout:14000});
  await expect(page.getByRole('link',{name:'Corresponding source ZIP',exact:true})).toBeVisible();await expect(page.locator('#run')).toBeDisabled();await page.unroute('**/worker.js');await page.locator('#retry').click();await expect(page.locator('#status')).toContainText('Rust report accepted');
});

test('real five-second file-read deadline retains draft originals and accepted report',async({page})=>{
  await page.addInitScript(()=>{const original=File.prototype.arrayBuffer;File.prototype.arrayBuffer=function(){return this.name==='MISSION.md'?new Promise(()=>{}):original.call(this);};});await open(page);const before=await accepted(page);const beforeDraft=await draft(page);
  await page.locator('#files').setInputFiles({name:'MISSION.md',mimeType:'text/plain',buffer:Buffer.from('unread')});await expect(page.locator('#read-status')).toContainText('File read exceeded 5 seconds',{timeout:7000});expect(await accepted(page)).toEqual(before);expect(await draft(page)).toEqual(beforeDraft);await expect(page.locator('#original-file')).toBeDisabled();
});

test('an edited draft wins over a late file read without leaving a reading status',async({page})=>{
  await page.addInitScript(()=>{const original=File.prototype.arrayBuffer;File.prototype.arrayBuffer=async function(){const bytes=await original.call(this);await new Promise(resolve=>setTimeout(resolve,600));return bytes;};});await open(page);const before=await draft(page);
  await page.locator('#files').setInputFiles({name:'MISSION.md',mimeType:'text/plain',buffer:Buffer.from('late file')});await page.locator('#editor').fill('newer TRACE draft');await page.waitForTimeout(800);
  const current=await draft(page);expect(current.files.find(file=>file.name==='MISSION.md')).toEqual(before.files.find(file=>file.name==='MISSION.md'));expect(current.files.find(file=>file.name==='TRACE.md').text).toBe('newer TRACE draft');await expect(page.locator('#read-status')).toContainText('superseded');await expect(page.locator('#original-file')).toBeDisabled();
});

test('loading the example fences an older file read and its original collection',async({page})=>{
  await page.addInitScript(()=>{const original=File.prototype.arrayBuffer;File.prototype.arrayBuffer=async function(){const bytes=await original.call(this);await new Promise(resolve=>setTimeout(resolve,600));return bytes;};});await open(page);
  await page.locator('#files').setInputFiles({name:'MISSION.md',mimeType:'text/plain',buffer:Buffer.from('old read')});await page.locator('#example').click();await expect(page.locator('#read-status')).toContainText('Synthetic example loaded');await page.waitForTimeout(800);
  expect((await draft(page)).files.find(file=>file.name==='MISSION.md').text).toContain('Synthetic package');await expect(page.locator('#original-file')).toBeDisabled();
});

test('worker exception retains accepted data and retry uses the real worker',async({page})=>{
  await page.route('**/worker.js',route=>route.fulfill({contentType:'text/javascript',body:init+`let count=0;self.onmessage=({data})=>{if(count++)throw Error('injected fault');postMessage({type:'result',id:data.id,report:JSON.parse(evaluate(data.request))});};`}));await open(page);const before=await accepted(page);await page.locator('#run').click();await expect(page.locator('#status')).toContainText('Worker error');expect(await accepted(page)).toEqual(before);
  await page.unroute('**/worker.js');await page.locator('#retry').click();await expect(page.locator('#run')).toBeEnabled();await page.locator('#run').click();await expect(page.locator('#status')).toContainText('Rust report accepted');
});

test('failed example fetching preserves current input and accepted result',async({page})=>{
  await open(page);const before=await accepted(page);const beforeDraft=await draft(page);await page.route('**/example.json',route=>route.abort());await page.locator('#example').click();await expect(page.locator('#read-status')).toContainText('Example load failed');expect(await accepted(page)).toEqual(before);expect(await draft(page)).toEqual(beforeDraft);
});
