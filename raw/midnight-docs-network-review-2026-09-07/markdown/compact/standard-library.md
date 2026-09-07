> For the complete documentation index, see [llms.txt](/llms.txt)

# Compact standard library

**CompactStandardLibrary** ∙ [Detailed API reference](/compact/standard-library/exports.md)

This API provides standard types and circuits for use in Compact programs. Key parts of the API are:

* Common data types:

  <!-- -->

  * [`Maybe`](/compact/standard-library/exports.md#maybe)
  * [`Either`](/compact/standard-library/exports.md#either)
  * [`NativePoint`](/compact/standard-library/exports.md#nativepoint)
  * [`Secp256k1Point`](/compact/standard-library/exports.md#secp256k1point)
  * [`MerkleTreeDigest`](/compact/standard-library/exports.md#merkletreedigest)
  * [`MerkleTreePathEntry`](/compact/standard-library/exports.md#merkletreepathentry)
  * [`MerkleTreePath`](/compact/standard-library/exports.md#merkletreepath)
  * [`ContractAddress`](/compact/standard-library/exports.md#contractaddress)
  * [`ZswapCoinPublicKey`](/compact/standard-library/exports.md#zswapcoinpublickey)
  * [`UserAddress`](/compact/standard-library/exports.md#useraddress)

* Coin management data types:

  <!-- -->

  * [`ShieldedCoinInfo`](/compact/standard-library/exports.md#shieldedcoininfo)
  * [`QualifiedShieldedCoinInfo`](/compact/standard-library/exports.md#qualifiedshieldedcoininfo)
  * [`ShieldedSendResult`](/compact/standard-library/exports.md#shieldedsendresult)

* Common functions:

  <!-- -->

  * [`some`](/compact/standard-library/exports.md#some)
  * [`none`](/compact/standard-library/exports.md#none)
  * [`left`](/compact/standard-library/exports.md#left)
  * [`right`](/compact/standard-library/exports.md#right)

* Hashing functions:

  <!-- -->

  * [`transientHash`](/compact/standard-library/exports.md#transienthash)
  * [`transientCommit`](/compact/standard-library/exports.md#transientcommit)
  * [`persistentHash`](/compact/standard-library/exports.md#persistenthash)
  * [`persistentCommit`](/compact/standard-library/exports.md#persistentcommit)
  * [`degradeToTransient`](/compact/standard-library/exports.md#degradetotransient)
  * [`upgradeFromTransient`](/compact/standard-library/exports.md#upgradefromtransient)
  * [`keccak256`](/compact/standard-library/exports.md#keccak256)

* Elliptic curve types and functions:

  <!-- -->

  * [`JubjubPoint`](/compact/standard-library/exports.md#jubjubpoint)
  * [`JubjubScalar`](/compact/standard-library/exports.md#jubjubscalar)
  * [`constructJubjubPoint`](/compact/standard-library/exports.md#constructjubjubpoint)
  * [`jubjubPointX`](/compact/standard-library/exports.md#jubjubpointx)
  * [`jubjubPointY`](/compact/standard-library/exports.md#jubjubpointy)
  * [`Secp256k1Point`](/compact/standard-library/exports.md#secp256k1point)
  * [`Secp256k1Base`](/compact/standard-library/exports.md#secp256k1base)
  * [`Secp256k1Scalar`](/compact/standard-library/exports.md#secp256k1scalar)
  * [`secp256k1PointX`](/compact/standard-library/exports.md#secp256k1pointx)
  * [`secp256k1PointY`](/compact/standard-library/exports.md#secp256k1pointy)
  * [`ecAdd`](/compact/standard-library/exports.md#ecadd)
  * [`ecNeg`](/compact/standard-library/exports.md#ecneg)
  * [`ecMul`](/compact/standard-library/exports.md#ecmul)
  * [`ecMulGenerator`](/compact/standard-library/exports.md#ecmulgenerator)
  * [`hashToCurve`](/compact/standard-library/exports.md#hashtocurve)

* secp256k1 field arithmetic functions:

  <!-- -->

  * [`neg`](/compact/standard-library/exports.md#neg)
  * [`inv`](/compact/standard-library/exports.md#inv)

* Merkle tree functions:

  <!-- -->

  * [`merkleTreePathRoot`](/compact/standard-library/exports.md#merkletreepathroot)
  * [`merkleTreePathRootNoLeafHash`](/compact/standard-library/exports.md#merkletreepathrootnoleafhash)

* Coin management functions:

  <!-- -->

  * [`tokenType`](/compact/standard-library/exports.md#tokentype)
  * [`nativeToken`](/compact/standard-library/exports.md#nativetoken)
  * [`ownPublicKey`](/compact/standard-library/exports.md#ownpublickey)
  * [`createZswapInput`](/compact/standard-library/exports.md#createzswapinput)
  * [`createZswapOutput`](/compact/standard-library/exports.md#createzswapoutput)
  * [`mintShieldedToken`](/compact/standard-library/exports.md#mintshieldedtoken)
  * [`evolveNonce`](/compact/standard-library/exports.md#evolvenonce)
  * [`receiveShielded`](/compact/standard-library/exports.md#receiveshielded)
  * [`sendShielded`](/compact/standard-library/exports.md#sendshielded)
  * [`sendImmediateShielded`](/compact/standard-library/exports.md#sendimmediateshielded)
  * [`mergeCoin`](/compact/standard-library/exports.md#mergecoin)
  * [`mergeCoinImmediate`](/compact/standard-library/exports.md#mergecoinimmediate)
  * [`shieldedBurnAddress`](/compact/standard-library/exports.md#shieldedburnaddress)
  * [`mintUnshieldedToken`](/compact/standard-library/exports.md#mintunshieldedtoken)
  * [`sendUnshielded`](/compact/standard-library/exports.md#sendunshielded)
  * [`receiveUnshielded`](/compact/standard-library/exports.md#receiveunshielded)
  * [`unshieldedBalance`](/compact/standard-library/exports.md#unshieldedbalance)
  * [`unshieldedBalanceLt`](/compact/standard-library/exports.md#unshieldedbalancelt)
  * [`unshieldedBalanceGte`](/compact/standard-library/exports.md#unshieldedbalancegte)
  * [`unshieldedBalanceGt`](/compact/standard-library/exports.md#unshieldedbalancegt)
  * [`unshieldedBalanceLte`](/compact/standard-library/exports.md#unshieldedbalancelte)
  * [`shieldedburnaddress`](/compact/standard-library/exports.md#shieldedburnaddress)

* Block time functions:

  <!-- -->

  * [`blockTimeLt`](/compact/standard-library/exports.md#blocktimelt)
  * [`blockTimeGte`](/compact/standard-library/exports.md#blocktimegte)
  * [`blockTimeGt`](/compact/standard-library/exports.md#blocktimegt)
  * [`blockTimeLte`](/compact/standard-library/exports.md#blocktimelte)

* Cryptographic signature types and functions:

  <!-- -->

  * [`JubjubSchnorrSignature`](/compact/standard-library/exports.md#jubjubschnorrsignature)
  * [`jubjubSchnorrVerify`](/compact/standard-library/exports.md#jubjubschnorrverify)
  * [`Secp256k1EcdsaSignature`](/compact/standard-library/exports.md#secp256k1ecdsasignature)
  * [`secp256k1EcdsaVerify`](/compact/standard-library/exports.md#secp256k1ecdsaverify)
  * [`secp256k1EthereumAddress`](/compact/standard-library/exports.md#secp256k1ethereumaddress)
