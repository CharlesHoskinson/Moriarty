[Skip to main content](#__docusaurus_skipToContent_fallback)

[![o1js](/o1js/img/o1js.svg)![o1js](/o1js/img/o1js.svg)](/o1js/)

[3.0.0](/o1js/advanced-concepts/recursion)

* [3.0.0](/o1js/advanced-concepts/recursion)
* [2.15.0](/o1js/2.15.0/advanced-concepts/recursion)
* [2.5.0](/o1js/2.5.0/advanced-concepts/recursion)
* [2.4.0](/o1js/2.4.0/advanced-concepts/recursion)

Search

* [Getting Started](/o1js/)

  + [Welcome](/o1js/)
  + [What is a ZK Constraint System?](/o1js/getting-started/what-is-a-zk-constraint-system)
* [Basic Types and Functions](/o1js/basic-types/field)

  + [Field](/o1js/basic-types/field)
  + [Integers](/o1js/basic-types/int)
  + [Keypairs and Signatures](/o1js/basic-types/keypairs-and-signatures)
  + [Merkle Trees](/o1js/basic-types/merkle-trees)
  + [Arrays](/o1js/basic-types/arrays)
  + [Structs](/o1js/basic-types/structs)
  + [Hashing](/o1js/basic-types/hashing)
* [Writing Constraint Systems](/o1js/writing-constraint-systems/witnesses)

  + [Witnesses](/o1js/writing-constraint-systems/witnesses)
  + [Conditional Logic](/o1js/writing-constraint-systems/conditional-logic)
  + [Analyzing Constraints](/o1js/writing-constraint-systems/analyzing-constraint-systems)
  + [Full Proof Flow with ZkProgram](/o1js/writing-constraint-systems/zk-program)
* [ZkApps](/o1js/zkapps/intro)
* [Advanced Concepts](/o1js/advanced-concepts/native-prover)

  + [Native Prover](/o1js/advanced-concepts/native-prover)
  + [Serialization](/o1js/advanced-concepts/serialization)
  + [Recursion](/o1js/advanced-concepts/recursion)
  + [Sideloading](/o1js/advanced-concepts/sideloaded-vks)
  + [ZkApps](/o1js/advanced-concepts/ZkApps/upgradability)
* [Tutorials](/o1js/tutorials/hmac/hmac-basics)
* [API Reference](/o1js/api-reference/Introduction)
* [Changelog](/o1js/changelog)

* Advanced Concepts
* Recursion

On this page

Recursive Zk Proofs
===================

Recursion is a powerful tool for solving complex problems with zk proofs. Any particular constraint system will be limited by
the lack of dynamic programming and conditional execution, but these shortcomings can be mitigated at the system architecture
level with recursion.

Fibonacci Example
-----------------

The Fibonacci sequence is a classic example of recursion. We can use recursive zk proofs to succinctly verify any element in
the sequence.

### Define the Fibonacci State Type

First thing's first, let's define a provable type to represent the state of a Fibonacci sequence. We'll track the index of the
sequence, and the last two numbers in the sequence. We'll use a `Struct` to represent the state, and we'll add some extra
functionality to the class that will be useful in the constraint system and tests.

```
class FibonacciState extends Struct({  
  index: UInt32,  
  n_1: UInt32,  
  n_2: UInt32,  
}) {  
  static empty() {  
    return new FibonacciState({  
      index: UInt32.zero,  
      n_1: UInt32.zero,  
      n_2: UInt32.zero,  
    });  
  }  
  assertEquals(other: FibonacciState) {  
    this.index.assertEquals(other.index);  
    this.n_1.assertEquals(other.n_1);  
    this.n_2.assertEquals(other.n_2);  
  }  
}
```

### Writing the Recursive Program

With the state data type defined, we can move on to writing a recursive program to prove execution of the Fibonacci sequence.
The program needs two methods: one to represent the base case, and one to represent the recursive case. The two reasons for
which we need a base case are: 1. to generate a proof when we don't have a previous one to build off of, and 2. to prove that
all other proofs are built off of the same base case.

```
const Fibonacci = ZkProgram({  
  name: "Fibonacci",  
  publicInput: FibonacciState,  
  publicOutput: FibonacciState,  
  
  methods: {  
    base: {  
      privateInputs: [],  
      method: async (_: FibonacciState) => {  
        return {  
          publicOutput: new FibonacciState({  
            index: UInt32.from(1),  
            n_1: UInt32.from(1),  
            n_2: UInt32.from(1),  
          }),  
        };  
      },  
    },  
    recursive: {  
      privateInputs: [SelfProof],  
      method: async (  
        input: FibonacciState,  
        // Note that the `SelfProof` syntax here means that the proof has public input: FibonacciState and public output: FibonacciState  
        previousProof: SelfProof<FibonacciState, FibonacciState>  
      ) => {  
        // Assert that the public input to this method matches the public output of the previous proof  
        input.assertEquals(previousProof.publicOutput);  
        // Verify the previous proof  
        previousProof.verify();  
  
        // Return the next Fibonacci number  
        return {  
          publicOutput: new FibonacciState({  
            index: input.index.add(1),  
            n_1: input.n_1.add(input.n_2),  
            n_2: input.n_1,  
          }),  
        };  
      },  
    },  
  },  
});
```

For more details on `SelfProof`, check out the [api reference](/o1js/api-reference/classes/SelfProof).

### Using the Fibonacci Program

Now that we have a program, we can use it to generate proofs. Here is some example code showing how to generate a proof and
interact with it recursively.

```
await Fibonacci.compile();  
const proof1 = await Fibonacci.base(FibonacciState.empty());  
const proof2 = await Fibonacci.recursive(  
  proof1.proof.publicOutput,  
  proof1.proof  
);  
const proof3 = await Fibonacci.recursive(  
  proof2.proof.publicOutput,  
  proof2.proof  
);  
const proof4 = await Fibonacci.recursive(  
  proof3.proof.publicOutput,  
  proof3.proof  
);  
const proof5 = await Fibonacci.recursive(  
  proof4.proof.publicOutput,  
  proof4.proof  
);
```

Queue Compression Example
-------------------------

For a more realistic example, let's say that we have an application with some queue of events to process, and we want to
provably process them in a batch. In the example below, let's say we just want to compute the sum, but also prove that
processed the entire queue.

### Fill a Queue with Data

In this example, the queue will look a little like a blockchain. Each event will include a value and a hash of the value
and the previous event hash. That way we can prove that we've processed the queue in order.

```
let lastHash = Poseidon.hash([UInt32.zero.value]);  
const blocks = [{ value: UInt32.zero, hash: lastHash }];  
for (let i = 0; i < 3; i++) {  
  const value = UInt32.from(Math.floor(Math.random() * 100));  
  const block = { value, hash: Poseidon.hash([lastHash, value.value]) };  
  blocks.push(block);  
  lastHash = block.hash;  
}
```

### Write the Recursive Program

For the program, we need a data type like in the Fibonacci example. In this case, we'll want to track the running sum,
the starting hash of the proof range, and the latest hash that has been processed. Like in the Fibonacci
example, we'll need a base case and a recursive case. The base case will be a sum of 0 and a hash of 0, and the
recursive case will add one value to the running sum and update the hashes.

```
class BlockProofOutputs extends Struct({  
  currentSum: UInt32,  
  startingHash: Field,  
  endingHash: Field,  
}) {}  
  
const SumBlocks = ZkProgram({  
  name: "SumBlocks",  
  publicInput: UInt32,  
  publicOutput: BlockProofOutputs,  
  
  methods: {  
    base: {  
      privateInputs: [],  
      method: async (_: UInt32) => {  
        return {  
          publicOutput: new BlockProofOutputs({  
            currentSum: UInt32.zero,  
            startingHash: Poseidon.hash([UInt32.zero.value]),  
            endingHash: Poseidon.hash([UInt32.zero.value]),  
          }),  
        };  
      },  
    },  
    addNext: {  
      privateInputs: [SelfProof],  
      method: async (  
        input: UInt32,  
        previousProof: SelfProof<UInt32, BlockProofOutputs>  
      ) => {  
        previousProof.verify();  
  
        return {  
          publicOutput: new BlockProofOutputs({  
            currentSum: previousProof.publicOutput.currentSum.add(input),  
            startingHash: previousProof.publicOutput.startingHash,  
            endingHash: Poseidon.hash([  
              previousProof.publicOutput.endingHash,  
              input.value,  
            ]),  
          }),  
        };  
      },  
    },  
  },  
});
```

### Using the Queue Program

Here's how you would use the queue program to generate a proof.

```
await SumBlocks.compile();  
  
const baseProof = await SumBlocks.base(UInt32.zero);  
let proof = baseProof;  
for (let i = 1; i < blocks.length; i++) {  
  const block = blocks[i];  
  const nextProof = await SumBlocks.addNext(block.value, proof.proof);  
  proof = nextProof;  
}
```

[Edit this page](https://github.com/o1-labs/o1js-documentation-site/edit/main/docs/advanced-concepts/recursion.mdx)

[Previous

Serialization](/o1js/advanced-concepts/serialization)[Next

Sideloading](/o1js/advanced-concepts/sideloaded-vks)

* [Fibonacci Example](#fibonacci-example)
  + [Define the Fibonacci State Type](#define-the-fibonacci-state-type)
  + [Writing the Recursive Program](#writing-the-recursive-program)
  + [Using the Fibonacci Program](#using-the-fibonacci-program)
* [Queue Compression Example](#queue-compression-example)
  + [Fill a Queue with Data](#fill-a-queue-with-data)
  + [Write the Recursive Program](#write-the-recursive-program)
  + [Using the Queue Program](#using-the-queue-program)

[Twitter](https://x.com/o1_labs)·[Discord](https://discord.gg/minaprotocol)·[GitHub](https://github.com/o1-labs/o1js)

Copyright © 2026 o1Labs