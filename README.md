# towerdefence-demo

## Nounce Convention
Allo command is composed of 3 parts

[0-8) bits are command number.
[8-16) bits are supplement attributes (Eg. Tower features).
[16-64) bits are nounce.

## API conventions
Step:
[command = 0, reserved = 0, reserved = 0, reserved = 0]

Place Tower:
[command = 1, TowerID: u64, Position: u64 = [u32, u32], reserved = 0]

Claim Tower:
[command = 2, TowerID: u64, reserved = 0, reserved = 0]

Mint Tower:
[command = 3 && (TowerFeature << 8), TowerID: u64, PubkeySecondU64: u64, PubkeyThirdU64 = 0]

Drop Tower:
[command = 4, TowerID: u64, reserved = 0, reserved = 0]

Upgrade Tower:
[command = 5, TowerID: u64, Recipie: u64, reserved = 0]

Place Modifier:
[command = 6, Modifier: u64, Position: u64, reserved = 0]

Remove Modifier:
[command = 7, Modifier: u64, Position: u64, reserved = 0]



## Signing Transactions
Each transaction should contains a structure of msg, pubkey, sign. The msg is a bignumber that is equivalent to a u64 array of length 4. This msg should be used to encode all the information of a user command. The pkx and pky are the pubkey of the user and the sig(x,y,r) is the signature of msg using the privateky that is related to the pubkey.

Please **use** the following tested function to sign a transaction:
```
export function sign(cmd: Array<bigint>, prikey: string) {
  let pkey = PrivateKey.fromString(prikey);
  let r = pkey.r();
  let R = Point.base.mul(r);
  let H = cmd[0] + (cmd[1] << 64n) + (cmd[2] << 128n) + (cmd[3] << 196n);
  let hbn = new BN(H.toString(10));
  let S = r.add(pkey.key.mul(new CurveField(hbn)));
  let pubkey = pkey.publicKey;
  const data = {
    msg: bnToHexLe(hbn),
    pkx: bnToHexLe(pubkey.key.x.v),
    pky: bnToHexLe(pubkey.key.y.v),
    sigx: bnToHexLe(R.x.v),
    sigy: bnToHexLe(R.y.v),
    sigr: bnToHexLe(S.v),
  };
  return data;
}
```


## State Encoding
see https://github.com/DelphinusLab/towerdefence-demo/blob/main/src/game/state.rs#L43
