> For the complete documentation index, see [llms.txt](https://hyperliquid.gitbook.io/hyperliquid-docs/llms.txt). Markdown versions of documentation pages are available by appending `.md` to page URLs; this page is available as [Markdown](https://hyperliquid.gitbook.io/hyperliquid-docs/hypercore/multi-sig.md).

# Multi-sig

Advanced Feature

HyperCore supports native multi-sig actions. This allows multiple private keys to control a single account for additional security. Unlike other chains, multi-sig is available as a built-in primitive on HyperCore as opposed to relying on smart contracts.&#x20;

The multi-sig workflow is described below:

* To convert a user to a multi-sig user, the user sends a `ConvertToMultiSigUser` action with the authorized users and the minimum number of authorized users required to sign an action. Authorized users must be existing users on Hyperliquid. Once a user has been converted into a multi-sig user, all its actions must be sent via multi-sig.&#x20;
* To send an action, each authorized user must sign a payload to produce a signature. A `MultiSig` action wraps around any normal action and includes a list of signatures from authorized users.&#x20;
* The `MultiSig` payload also contains the target multi-sig user and the authorized user who will ultimately send the `MultiSig` action to the blockchain. The sender of the final action is also known as the `leader` (transaction lead address) of the multi-sig action.
  * When a multi-sig action is sent, only the nonce set of the authorized user who sent the transaction is validated and updated.
  * Similar to normal actions, the leader can also be an API wallet of an authorized user. In this case, the nonce of the API wallet is checked and updated.&#x20;
* A multi-sig user's set of authorized users and/or the threshold may be updated by sending a `MultiSig` action wrapping a `ConvertToMultiSigUser` action describing the new state.
* A multi-sig user can be converted back to a normal user by sending a `ConvertToMultiSigUser` via multi-sig. In this case, the set of authorized users can be set to empty and conversion to a normal user will be performed.

Miscellaneous notes:&#x20;

* The leader (transaction lead address) must be an authorized user, not the multi-sig account
* Each signature must use the same information, e.g., same nonce, transaction lead address, etc.&#x20;
* The leader must collect all signatures before submitting the action&#x20;
* A user can be a multi-sig user and an authorized user for another multi-sig user at the same time. A user may be an authorized user for multiple multi-sig users. The maximum allowed number of authorized users for a given multi-sig user is 10.&#x20;

Important for HyperEVM users: Converting a user to a multi-sig still leaves the HyperEVM user controllable by the original wallet. CoreWriter does not work for multi-sig users. In general, multi-sig users should not interact with the HyperEVM before or after conversion.

See the Python SDK for code examples.
