Get circuit with Summon:

```ts
export default (io: Summon.IO) => {
  const input1 = io.input("alice", "input1", summon.number());
  const input2 = io.input("alice", "input2", summon.number());

  let res = input1 * input2;

  io.outputPublic("res", res);
};
```

```bash
./target/debug/summonc ./examples/simple.ts --boolify-width 8
```
