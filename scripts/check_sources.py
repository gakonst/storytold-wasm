"""Verify extracted source dependencies stay inside this repository."""
import json,pathlib,tomllib,os
root=pathlib.Path(__file__).resolve().parent.parent
components=json.loads((root/'components.json').read_text())
for name,component in components.items():
 folder=root/component['path'];manifest=json.loads((folder/'package.json').read_text())
 assert manifest['name']==component['package'],name
 for script in ['build','test']:assert script in manifest['scripts'],(name,script)
 assert (folder/'wrangler.jsonc').exists(),name
ignored={'target','node_modules','pkg','dist','dist-deploy','generated','.git'};count=0
for base,dirs,files in os.walk(root/'packages'):
 dirs[:]=[d for d in dirs if d not in ignored and not d.startswith('target-')]
 if 'Cargo.toml' not in files:continue
 file=pathlib.Path(base)/'Cargo.toml';doc=tomllib.loads(file.read_text());count+=1
 def walk(value):
  if isinstance(value,dict):
   for key,item in value.items():
    if key=='path' and isinstance(item,str):
     resolved=(file.parent/item).resolve()
     assert resolved.is_relative_to(root),f'{file}: external path {item}'
     assert resolved.exists(),f'{file}: missing {item}'
    else:walk(item)
  elif isinstance(value,list):
   for item in value:walk(item)
 walk(doc)
print(f'Checked {len(components)} Worker packages and {count} Rust manifests; all referenced paths exist within the repository.')
