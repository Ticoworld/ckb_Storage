"""Local proof reconstruction from retained user state and an untrusted public archive.
No operator tree/database API. This is not a CKB light client: the initial cell and
expected live outpoint must be independently authenticated by the caller.
"""
import hashlib
import json
import sys
from cryptography.hazmat.primitives.asymmetric.ed25519 import Ed25519PublicKey

LIMIT=(1<<64)-1

def h(b):
    return hashlib.blake2b(b,digest_size=32,person=b'ckb-default-hash').digest()

def u32(b):
    return int.from_bytes(b,'little')

def parts(b):
    assert len(b)>=4 and u32(b[:4])==len(b), 'bad molecule size'
    if len(b)==4:
        return []
    first=u32(b[4:8]);assert first>=8 and first%4==0
    count=first//4-1
    offsets=[u32(b[4+4*i:8+4*i]) for i in range(count)]+[len(b)]
    assert offsets[0]==first and offsets==sorted(offsets) and offsets[-1]==len(b)
    return [b[offsets[i]:offsets[i+1]] for i in range(count)]

def raw_bytes(b):
    assert len(b)>=4 and u32(b[:4])==len(b)-4
    return b[4:]

def node(n):
    return bytes.fromhex(n['hash']),int(n['sum'])

def enc(n):
    return n[0]+n[1].to_bytes(8,'little')

def parent(l,r):
    total=l[1]+r[1];assert total<=LIMIT,'sum overflow'
    return h(b'\x01'+enc(l)+enc(r)),total

def leaf(k,a):
    return h(b'\x00'+k+a.to_bytes(8,'little')),a

def fold(n,i,p):
    for level,s in enumerate(p):
        n=parent(n,s) if (i>>level)&1==0 else parent(s,n)
    return n

def parse_state(b):
    assert len(b)==41
    return b[0],(b[1:33],int.from_bytes(b[33:],'little'))

def reconstruct(local,archive):
    own=local['index'];amount=local['amount_shannons'];key=bytes.fromhex(local['pubkey'])
    proof=[node(n) for n in local['proof']]
    depth,current=parse_state(bytes.fromhex(local['initial_root_data']))
    assert len(proof)==depth and fold(leaf(key,amount),own,proof)==current
    outpoint=local['initial_outpoint'];updates=0
    pool_type=bytes.fromhex(local['type_script'])
    for encoded in archive['packed_transactions']:
        tx=parts(bytes.fromhex(encoded));assert len(tx)==2
        raw=parts(tx[0]);assert len(raw)==6
        inputs=raw[3];assert len(inputs)==4+44*u32(inputs[:4])
        assert inputs[12:48].hex()==outpoint,'missing, reordered or wrong-instance history'
        witnesses=parts(tx[1]);wa=parts(raw_bytes(witnesses[0]));assert len(wa)==3
        w=raw_bytes(wa[1]);assert len(w)==109+40*depth and w[0] in (1,2)
        spent=u32(w[1:5]);a=int.from_bytes(w[5:13],'little');k=w[13:45]
        p=[(w[45+40*i:77+40*i],int.from_bytes(w[77+40*i:85+40*i],'little')) for i in range(depth)]
        assert spent < 1<<depth and spent!=own and a>10_000
        assert fold(leaf(k,a),spent,p)==current,'forged membership/root/sum'
        Ed25519PublicKey.from_public_bytes(k).verify(w[-64:],h(tx[0]))
        updated=fold((h(b'\x02'),0),spent,p)
        assert updated[1]==current[1]-a
        output=parts(parts(raw[4])[0]);data=raw_bytes(parts(raw[5])[0])
        assert output[2]==pool_type,'instance/type substitution'
        assert int.from_bytes(output[0],'little')==updated[1]+147*100_000_000
        assert parse_state(data)==(depth,updated),'wrong successor'
        divergence=(own^spent).bit_length()-1
        subtree=(h(b'\x02'),0)
        for level in range(divergence):
            subtree=parent(subtree,p[level]) if (spent>>level)&1==0 else parent(p[level],subtree)
        proof[divergence]=subtree
        assert fold(leaf(key,amount),own,proof)==updated
        current=updated;outpoint=(h(tx[0])+bytes(4)).hex();updates+=1
    assert outpoint==archive['independently_authenticated_live_outpoint'],'incomplete or forked history'
    return {'index':own,'amount_shannons':amount,'pubkey':key.hex(),
            'proof':[{'hash':n[0].hex(),'sum':n[1]} for n in proof],
            'live_outpoint':outpoint,'live_root_data':(bytes([depth])+enc(current)).hex(),
            'public_transitions_processed':updates}

if __name__=='__main__':
    try:
        result=reconstruct(json.load(open(sys.argv[1])),json.load(open(sys.argv[2])))
        print(json.dumps(result,indent=2))
    except Exception as exc:
        print(f'REJECTED PUBLIC ARCHIVE: {type(exc).__name__}: {exc}',file=sys.stderr)
        sys.exit(1)
