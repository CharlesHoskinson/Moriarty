> For the complete documentation index, see [llms.txt](https://docs.essential.builders/llms.txt). Markdown versions of documentation pages are available by appending `.md` to page URLs; this page is available as [Markdown](https://docs.essential.builders/protocol-overview/contracts-predicates-and-constraints.md).

# Contracts, Predicates, and Constraints

As on other blockchains, at the highest level, state on Essential belongs to *contracts* which declare that state. Although this terminology will be familiar to developers coming from imperative blockchain languages (e.g. Solidity), a *declarative* contract is a fundamentally different thing.

Imperative "smart contracts" take a set of inputs, and update state as a side-effect of the execution of a sequence of opcodes over these inputs. In particular, a set of storage opcodes exist which directly act upon state. Conversely, we have stated that Essential achieves state updates "*declaratively*"; without the need for execution. This means that from the point of view of an Essential application, things happen in reverse when compared to the imperative approach. We start with a (proposed) atomic state mutation (i.e. a set of proposed new state values), and then substitute those values into a contract to check their validity.

Validation occurs through the satisfaction of one of a contract's predicates. You can think of predicates as "pathways to validity" for the contract: in order for the contract to be satisfied (and therefore, its state mutated), *one* of its predicates must be satisfied.

A predicate is a block of code comprising one or more constraints. A constraint is simply a boolean expression which must evaluate to `True` for the predicate containing it to be satisfied. From a code organization point of view, a predicate may look a bit like a function. However, the distinction is very important. A predicate is in no sense "called" in the same way a function is. It is simply a target that individual solutions may seek to satisfy. Note that this process is explicit : a solution must identify which predicate it is satisfying in order to mutate the state. Individual predicates, as well as the contracts which contain them, are content-addressed.

A contract does not necessarily have to define a storage block. It may simply apply additional constraints to state mutations on other contracts. In this case, both the constraints of this contract *and the constraints contract which owns the state* must be satisfied for a solution to be valid. This allows for efficient re-use of deployed code. For example, the constraints governing a token swap can be deployed once, and used multiple times by other users and applications. This works because users can provide User Data to further constrain the solution space of a predicate, which we will see in the chapter on User Data and Solutions.

\
&#x20;
