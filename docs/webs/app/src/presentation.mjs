/** Exact reviewer-approved substitutions; this adapter never edits source inputs. */
export function applyPresentationPatches(sourceData, specification) {
  const presentation=structuredClone(sourceData);
  for(const patch of specification.patches) {
    const parts=patch.sourceField.split('/').slice(1).map(part=>part.replace(/~1/g,'/').replace(/~0/g,'~'));
    const field=parts.pop();
    const target=parts.reduce((value,key)=>value?.[key],presentation[patch.dataKey]);
    const before=target?.[field];
    if(typeof before!=='string'||!patch.old||before.split(patch.old).length!==2)throw new Error(`Presentation patch ${patch.id}: expected one exact source occurrence.`);
    const index=before.indexOf(patch.old);
    target[field]=before.slice(0,index)+patch.new+before.slice(index+patch.old.length);
  }
  return presentation;
}
