import { PublicKey, type Resource } from "@anomaorg/arm-bindings";
import { toBase64 } from "primitives";
import type { Parameters, Permit2Data, UserKeyring } from "types";
import type { Address } from "viem";
import { TransferLogic } from "./TransferLogic";

/**
 * Build required backend Parameters request for mint, transfer, burn
 */
export class TransferBuilder {
  readonly client: TransferLogic;

  constructor(client: TransferLogic) {
    this.client = client;
  }

  static async init(
    transferLogicVerifyingKey: string,
    trivialLogicVerifyingKey: string,
    wasmBytes?: Uint8Array
  ): Promise<TransferBuilder> {
    const client = await TransferLogic.init(
      transferLogicVerifyingKey,
      trivialLogicVerifyingKey,
      wasmBytes ? new Uint8Array(wasmBytes) : undefined
    );
    return new TransferBuilder(client);
  }

  /** Builds the backend Parameters payload for a mint (deposit) transaction. */
  buildMintParameters(
    mintResources: {
      createdResource: Resource;
      consumedResource: Resource;
    },
    permit2Data: Permit2Data,
    evmAddress: Address,
    tokenContractAddress: Address,
    keyring: UserKeyring
  ): Parameters {
    const { createdResource, consumedResource } = mintResources;

    return {
      consumedResources: [
        {
          resource: consumedResource.encode(),
          nullifierKey: toBase64(keyring.nullifierKeyPair.nk),
          witnessData: {
            TokenTransferEphemeralWrap: {
              permit2Data,
              senderWalletAddress: evmAddress,
              tokenContractAddress,
            },
          },
        },
      ],
      createdResources: [
        {
          resource: createdResource.encode(),
          witnessData: {
            TokenTransferPersistent: {
              receiverDiscoveryPublicKey: new PublicKey(
                keyring.discoveryKeyPair.publicKey
              ).toBase64(),
              receiverEncryptionPublicKey: new PublicKey(
                keyring.encryptionKeyPair.publicKey
              ).toBase64(),
              receiverAuthorizationVerifyingKey: new PublicKey(
                keyring.authorityKeyPair.publicKey
              ).toBase64(),
              tokenContractAddress,
            },
          },
        },
      ],
    };
  }
}
