import {test,expect} from '@playwright/test';
import crypto from 'node:crypto';
test('all served files match inventory and legal/source routes survive engine failure',async({page,request})=>{
  const inventory=await(await request.get('inventory.json')).json();
  for(const file of inventory.files){const response=await request.get(file.path);expect(response.status(),file.path).toBe(200);const bytes=await response.body();expect(bytes.length,file.path).toBe(file.bytes);expect(crypto.createHash('sha256').update(bytes).digest('hex'),file.path).toBe(file.sha256);if(file.path.endsWith('.wasm'))expect(response.headers()['content-type']).toContain('application/wasm');}
  await page.route('**/worker.js',route=>route.abort());await page.goto('./');
  for(const name of ['Native Rust','Browser source','Corresponding source ZIP','Compiled WASM','Build','Licenses & scope','Build record'])await expect(page.getByRole('link',{name,exact:true})).toBeVisible();
  await page.getByRole('link',{name:'Licenses & scope',exact:true}).click();await expect(page.locator('h1')).toContainText('VTRACE package inspector notices');
  for(const link of await page.locator('a').evaluateAll(nodes=>nodes.map(x=>x.getAttribute('href')).filter(x=>!x.startsWith('http'))))expect((await request.get(link)).ok(),link).toBe(true);
});
test('complete corresponding snapshot includes native fixtures and retains split grant scope',async({request})=>{
  const build=await(await request.get('build.json')).json();const files=await(await request.get('source-files.json')).json();
  for(const path of ['src/main.rs','tests/cli.rs','tests/in_memory.rs','docs/vtrace/EVIDENCE.md','docs/vtrace/pitfall-integration-boundaries.v1.json','.roles/ROLE.md','templates/adoption/MISSION.md','browser/package-inspector/web/app.js'])expect(files.some(file=>file.path===path),path).toBe(true);
  expect(build.full_git_archive).toContain(build.commit);const source=await(await request.get('source.zip')).body();expect(crypto.createHash('sha256').update(source).digest('hex')).toBe(build.source_zip_sha256);
  const scope=await(await request.get('SOURCE-SCOPE.md')).text();expect(scope).toContain('CC BY-NC');expect(scope).toContain('every tracked Git blob');expect(scope).toContain('No NASA handbook');
});
