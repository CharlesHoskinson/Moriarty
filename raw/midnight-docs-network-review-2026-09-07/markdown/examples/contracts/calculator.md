# Calculator Contract

> For the complete documentation index, see [llms.txt](/llms.txt)

This Compact contract implements a simple calculator. It offers demonstration of the following features:

* Witness function
* On-chain verification of off-chain compute

Notice that `divMod` only has the declaration of the function signature. Its logic is implemented in the Typescript frontend and is unknown to contract, though we can enforce expected execution through `assert` statements in `divide`.

```
pragma language_version 0.23;



export ledger result: Uint<16>;



export circuit add(num1: Uint<16>, num2: Uint<16>): [] {

    result = disclose(num1 + num2 as Uint<16>);

}



export circuit subtract(num1: Uint<16>, num2: Uint<16>): [] {

    result = disclose(num1 - num2 as Uint<16>);

}



export circuit multiply(num1: Uint<16>, num2: Uint<16>): [] {

    result = disclose(num1 * num2 as Uint<16>);

}



export circuit square(num1: Uint<16>): [] {

    result = disclose(num1 * num1 as Uint<16>);

}



// declare an off-chain function expected to divide two numbers

witness divMod(num1: Uint<16>, num2: Uint<16>): [Uint<16>, Uint<16>];



// enforce correct logic on-chain for off-chain logic in divMod

circuit divide(num1: Uint<16>, num2: Uint<16>): Uint<16> {

    const [quo, rem] = divMod(num1, num2);

    assert(rem < num2 && quo * num2 + rem == num1, "incorrect division");

    return quo;

}
```
