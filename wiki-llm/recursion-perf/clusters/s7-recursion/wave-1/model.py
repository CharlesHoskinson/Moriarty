"""ESTIMATE only: recorded geometry and conditional cost arithmetic; no proving."""
from pathlib import Path
import json, math
A,B,PAIR,TABLE=190727,191842,316833,1032465
L,N,D=163.630577,867.279750,586.381690

def domain(rows, table=TABLE):
    return math.ceil(math.log2(max(rows,table)+7))

def node(b, terminal=False):
    rows=A+B*b+(PAIR if terminal else 0)
    k=domain(rows)
    return dict(arity=b,terminal=terminal,rows=rows,k=k,domain_units=2**(k-20),fft_units=2**(k-20)*k/20)

# A structural toy DP: every child is charged the homogeneous leaf-verifier fit.
# Actual recursive children have different VK/architecture; these are NOT predictions.
def tree_dp(n, terminal=False):
    from functools import lru_cache
    @lru_cache(None)
    def parts(total,count):
        if count==1: return [(total,)]
        return [(first,)+tail for first in range(1,total-count+2) for tail in parts(total-first,count-1) if first<=tail[0]]
    @lru_cache(None)
    def best(m, root):
        if m==1 and not root:return (0,'leaf')
        choices=[]
        for b in range(2,m+1):
            r=node(b,root)
            for p in parts(m,b):
                sub=[best(x,False) for x in p]
                choices.append((r['domain_units']+sum(x[0] for x in sub),f"{'D' if root else 'N'}{b}({','.join(x[1] for x in sub)})"))
        return min(choices)
    return best(n,terminal)

out={
 'label':'ESTIMATE: arithmetic, not new proof measurements',
 'inputs':{'node_intercept':A,'node_marginal':B,'pairing_rows':PAIR,'table_rows':TABLE,'historical_leaves_s':L,'historical_node_s':N,'historical_decider_s':D},
 'node_geometry':[node(b,t) for t in (False,True) for b in (2,3,4,5,8,9,10,16)],
 'historical':{'total':L+N+D,'ten_x_target':(L+N+D)/10,'parent_only_ten_x_gain':(L+N+D)/(L+N/10+D),'free_node_gain':(L+N+D)/(L+D),'free_leaves_and_node_gain':(L+N+D)/D,'mono16_domain_proxy_seconds':L,'mono16_fft_proxy_seconds':L*19/17,'mono16_domain_proxy_gain':(L+N+D)/L,'mono16_fft_proxy_gain':(L+N+D)/(L*19/17)},
 'fusion':[{'k':k,'bulk_ratio':2**(20-k),'max_unscaled_share_for_10x':(.1-1/(2**(20-k)))/(1-1/(2**(20-k)))} for k in (15,16,17,19)],
 'fixed_pair_k15':[{'unscaled_share':h,'gain':1/(h+(1-h)/32),'seconds_against_A3':[t*(h+(1-h)/32) for t in (315.88,382.278)]} for h in (.02,.06,.10)],
 'folding_wrapper_budgets':[{'steps':16,'per_step_s':f,'compression_s':20,'max_wrapper_s_for_10x':(L+N+D)/10-16*f-20} for f in (1,5)],
 'toy_tree_dp':{str(n):{'separate_root_cost':tree_dp(n,False)[0]+1,'separate_root_shape':tree_dp(n,False)[1], 'fused_root_cost':tree_dp(n,True)[0],'fused_root_shape':tree_dp(n,True)[1]} for n in (4,16)},
 'caveats':['DP reuses leaf-only geometry for node children: toy structural proxy, not a proven bound or implemented recursion.','Row addition ignores overlaps and new binding glue.','Domain scaling assumes identical width/degree/phase mix; neither fixed nor proven.','16 transactions = 4 current leaves, not 16 leaves.']
}
path=Path(__file__).with_name('model-results.json')
path.write_text(json.dumps(out,indent=2)+'\n')
print(json.dumps(out,indent=2))
