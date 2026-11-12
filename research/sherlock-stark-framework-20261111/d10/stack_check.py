import json,sys
def width(l_skip, lsh, sorted_):
    col=row=0
    for w,lh in sorted_:
        for _ in range(w):
            ln = 1<<max(lh,l_skip)
            if row+ln > (1<<lsh):
                col+=1; row=0
            row+=ln
    return col + (1 if row else 0)
def ws(airs, l_skip=2, n_stack=18):
    s=sorted(airs, key=lambda a:-a[1])
    c=width(l_skip,l_skip+n_stack,[(w,lh) for w,lh,_ in s])
    o=sum(width(l_skip,l_skip+n_stack,[(cw,lh)]) for _,lh,cws in s for cw in cws)
    return c,o
d=json.load(open(sys.argv[1]))
for v in ['L_cached_bn254_ud','L_nocached_bn254_ud']:
    A=[(a['width_common'],a['log_height'],a['width_cached']) for a in d[v]['shape']['airs']]
    print(v,'measured',ws(A),'json',d[v]['shape']['stacked_width_common'],d[v]['shape']['stacked_width_other'])
    for lab,extra in [('+log 69 cols @2^8',[(20,8,[]),(49,8,[])]),('+log 69 cols @2^10',[(20,10,[]),(49,10,[])])]:
        print('  ',lab,ws(A+extra))
    B=[(w,lh+1 if n.startswith('Poseidon2Air') else lh,c) for (w,lh,c),n in zip(A,[a['name'] for a in d[v]['shape']['airs']])]
    print('   Poseidon2Air doubled + log @2^10', ws(B+[(20,10,[]),(49,10,[])]))
    cells=sum(a['width_common']*a['height'] for a in d[v]['shape']['airs'])
    print('   common cells',cells, 'cells/2^20', cells/2**20)
