/**
 * Cambrian demo-transfer payload
 * Creates a simple SOL transfer instruction from the executor PDA
 */
import { TPayloadInput } from '@cambrianone/commons';
import { address, createNoopSigner, getBase58Codec, lamports } from '@solana/web3.js';
import { getTransferSolInstruction } from '@solana-program/system';

const run = async ({
  executorPDA,
}: TPayloadInput): Promise<void> => {
  try {
    const { data, ...rest } = getTransferSolInstruction({
      source: createNoopSigner(address(executorPDA)),
      // Example recipient address
      destination: address('DnXet6kPAWkk2bjC55wvqKkRKkLcMAvdGxeAniNyM2GY'),
      // Transfer 0.005 SOL (5,000,000 lamports)
      amount: lamports(5000000n),
    });

    const res = {
      proposalInstructions: [{
        data: getBase58Codec().decode(data),
        ...rest,
      }],
    };
    console.log(JSON.stringify(res));
  } catch (e) {
    console.error('Payload error', e);
    throw e;
  }
};

const input = JSON.parse(process.env.CAMB_INPUT ?? '{}');

run(input).catch(e => {
  console.error('Error', e);
});
