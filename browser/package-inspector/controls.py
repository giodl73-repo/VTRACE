"""Owned synthetic requests and in-repo packages; no external source downloads."""
import json

NAMES = [
    'MISSION.md','REQUIREMENTS.md','SPECIFICATION_BASELINE.md','TRACE.md',
    'VERIFICATION.md','REVIEW.md','EVIDENCE.md','WORK_PACKAGES.md',
    'REVIEW_CHECKLISTS.md','LANGUAGE_PROFILES.md','COMMUNICATIONS_STRATEGY.md',
]

def document(name,text):
    return {'name':name,'text':text}

def sized(n):
    return ('x'*1023+'\n')*(n//1024)+('x'*(n%1024-1)+'\n' if n%1024 else '')

def requests(root,companion):
    cases=[{'files':[]}]
    for folder in [root/'docs/vtrace',root/'examples/existing-repo-migration/docs/vtrace']:
        # Repository fixtures use canonical LF for checkout/archive replay.
        # Caller CRLF/BOM/lone-CR custody is exercised separately and never normalized.
        cases.append({'files':[document(name,(folder/name).read_text(encoding='utf-8')) for name in NAMES if (folder/name).exists()]})
    cases.extend([{'files':[document('REQUIREMENTS.md','| ID | Text |\r\n|---|---|\r\n| REQ-001 | owned |\r\n'),document('TRACE.md',text)]} for text in ['missing','REQ-001','REQ-0010']])
    cases.extend([
        {'files':[document(name,'') for name in NAMES]},
        {'files':[document('MISSION.md','<script>alert(1)</script>\ufeff\r\n')]},
        {'files':[document('WORK_PACKAGES.md','| ID | Objective | Status | Extra |\n|---|---|---|---|\n| WP-001 | <img src=x> | passed | retain |\n')]},
        {'files':[document('WORK_PACKAGES.md','| ID |\n|---|\n| WP-💡\rMID\u2028END |\n')]},
        '{','null','{}','{"files":[],"files":[]}',
        '{"files":[{"name":"MISSION.md","text":"\\ud800"}]}',
        {'files':[],'extra':True},
        {'files':[{'name':'TRACE.md','text':'','extra':True}]},
    ])
    for files in [[document('../TRACE.md','')],[document('trace.md','')],[document('TRACE.md',''),document('TRACE.md','other')],[document('x'*65,'')],[document('MISSION.md','')]*12]:
        cases.append({'files':files})
    for text in [sized(65536),sized(65537),'é'*1024,'é'*1025,'\n'*2048,'\n'*2049,'é'*600000]:
        cases.append({'files':[document('MISSION.md',text)]})
    maximum=[document(name,sized(65536)) for name in NAMES[:4]]
    cases.extend([{'files':maximum},{'files':maximum+[document('REVIEW.md','x')]}])
    header='| ID | Parent IDs | Entry Criteria | Exit Criteria | L0 / L1 / L2 |\n|---|---|---|---|---|\n'
    row='| WP-'+'\0'*1800+' | | | | |\n'
    cases.append({'files':[document('WORK_PACKAGES.md',header+row*35)]})
    base='{"files":[]}'
    cases.extend([base+' '*(1048576-len(base)),base+' '*(1048577-len(base))])
    example=json.loads((companion/'web/example.json').read_text(encoding='utf-8'))
    cases.append(example)
    corrected=json.loads(json.dumps(example))
    for file in corrected['files']:
        if file['name']=='TRACE.md':file['text']=file['text'].replace('REQ-002','REQ-001')
    cases.append(corrected)
    return cases
