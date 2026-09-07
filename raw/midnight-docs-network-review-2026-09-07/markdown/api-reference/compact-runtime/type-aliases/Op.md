# Op

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.19.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / Op

# Type Alias: Op\<R>

```
type Op<R> = 

  | {

  noop: {

     n: number;

  };

}

  | "lt"

  | "eq"

  | "type"

  | "size"

  | "new"

  | "and"

  | "or"

  | "neg"

  | "log"

  | "root"

  | "pop"

  | {

  popeq: {

     cached: boolean;

     result: R;

  };

}

  | {

  addi: {

     immediate: number;

  };

}

  | {

  subi: {

     immediate: number;

  };

}

  | {

  push: {

     storage: boolean;

     value: EncodedStateValue;

  };

}

  | {

  branch: {

     skip: number;

  };

}

  | {

  jmp: {

     skip: number;

  };

}

  | "add"

  | "sub"

  | {

  concat: {

     cached: boolean;

     n: number;

  };

}

  | "member"

  | {

  rem: {

     cached: boolean;

  };

}

  | {

  dup: {

     n: number;

  };

}

  | {

  swap: {

     n: number;

  };

}

  | {

  idx: {

     cached: boolean;

     path: Key[];

     pushPath: boolean;

  };

}

  | {

  ins: {

     cached: boolean;

     n: number;

  };

}

  | "ckpt";
```

An individual operation in the onchain VM

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### R[​](#r "Direct link to R")

`R`

`null` or [AlignedValue](/api-reference/compact-runtime/type-aliases/AlignedValue.md), for gathering and verifying mode respectively
