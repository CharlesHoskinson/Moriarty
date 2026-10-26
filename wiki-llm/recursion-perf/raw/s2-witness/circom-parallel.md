# Circom templates and parallel components
> Source: https://raw.githubusercontent.com/iden3/circom/master/mkdocs/docs/circom-language/templates-and-components.md
> Collected: 2026-10-26
> Published: Unknown

# Templates & Components
## Templates
The mechanism to create generic circuits in Circom is the so-called templates.
They are normally parametric on some values that must be instantiated when the template is used. The instantiation of a template is a new circuit object, which can be used to compose other circuits, so as part of larger circuits. Since templates define circuits by instantiation, they have their own signals \(input, output, etc\).
```text
template tempid ( param\_1, ... , param\_n ) {
signal input a;
signal output b;
.....
}
```
Templates cannot include local functions or template definitions.
Assigning a value to an input signal inside the same template where it has been defined also generates the error \_"Exception caused by invalid assignment"\_ as can be seen in the next example.
```text
pragma circom 2.0.0;
template wrong (N) {
signal input a;
signal output b;
a <== N;
}
component main = wrong(1);
```
The instantiation of a template is made using the keyword component and by providing the necessary parameters.
```text
component c = tempid(v1,...,vn);
```
The values of the parameters should be known constants at compile time. The next code produces this compilation error message: \_"Every component instantiation must be resolved during the constraint generation phase".\_
```text
pragma circom 2.0.0;
template A(N1,N2){
signal input in;
signal output out;
out <== N1 \* in \* N2;
}
template wrong (N) {
signal input a;
signal output b;
component c = A(a,N);
}
component main {public [a]} = wrong(1);
```
## Components
A component defines an arithmetic circuit and, as such, it receives N input signals and produces M output signals and K intermediate signals. Additionally, it can produce a set of constraints.
In order to access the input or output signals of a component, we will use \*\*dot notation\*\*. No other signals are visible outside the component.
```text
c.a <== y\*z-1;
var x;
x = c.b;
```
The \*\*component instantiation\*\* will not be triggered until all its input signals are assigned to concrete values. Therefore the instantiation might be delayed and hence the component creation instruction does not imply the execution of the component object, but the creation of the instantiation process that will be completed when all the inputs are set. The output signals of a component can only be used when all inputs are set, otherwise a compiler error is generated. For instance, the following piece of code would result in an error:
```text
pragma circom 2.0.0;
template Internal() {
signal input in[2];
signal output out;
out <== in[0]\*in[1];
}
template Main() {
signal input in[2];
signal output out;
component c = Internal ();
c.in[0] <== in[0];
c.out ==> out; // c.in[1] is not assigned yet
c.in[1] <== in[1]; // this line should be placed before calling c.out
}
component main = Main();
```
\*\*Components are immutable\*\* (like signals). A component can be declared first and initialized in a second step. If there are several initialization instructions (in different execution paths) they all need to be instantiations of the same template (maybe with different values for the parameters).
```text
template A(N){
signal input in;
signal output out;
out <== in;
}
template C(N){
signal output out;
out <== N;
}
template B(N){
signal output out;
component a;
if(N > 0){
a = A(N);
}
else{
a = A(0);
}
a.in <== 1;
a.out ==> out;
}
component main = B(1);
```
If the instruction `a = A(0);`is replaced with `a = C(0)`, the compilation fails and the next error message is shown: \_"Assignee and assigned types do not match"\_.
We can define \*\*arrays of components\*\* following the same restrictions on the size given before. Moreover, initialization in the definition of arrays of components is not allowed, and instantiation can only be made component by component, accessing the positions of the array. All components in the array have to be instances of the same template as it can be seen in the next example.
```text
template MultiAND(n) {
signal input in[n];
signal output out;
component and;
component ands[2];
var i;
if (n==1) {
out <== in[0];
} else if (n==2) {
and = AND();
and.a <== in[0];
and.b <== in[1];
out <== and.out;
} else {
and = AND();
var n1 = n\2;
var n2 = n-n\2;
ands[0] = MultiAND(n1);
ands[1] = MultiAND(n2);
for (i=0; i