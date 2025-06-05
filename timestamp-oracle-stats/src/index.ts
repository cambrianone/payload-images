/**
 * Cambrian check-oracle payload implementation
 * Validates oracle data for Cambrian AVS (Actively Validated Services) and generates corresponding instructions
 */
import { getCheckOracleInstructionDataCodec } from '@cambrianone/oracle-client';
import { AccountRole, address, Endian, getBase58Codec, getProgramDerivedAddress, getU64Codec, getUtf8Codec } from '@solana/kit';

const run = async (_input: any): Promise<void> => {
  try {
    const { poaName, proposalStorageKey } = _input;

    // Storage space for Cambrian proposal (3 instructions × 25 bytes)
    const storageSpace = 3 * 25;

    // Cambrian threshold signature program address
    const SOLANA_THRESHOLD_SIGNATURE_PROGRAM_PROGRAM_ADDRESS = address('CsHC9SCyZWVg2aBNCe8iZTcxdBHZYEE68uMueTxLkniy');

    // Cambrian oracle program address
    const ORACLE_PROGRAM_PROGRAM_ADDRESS = address('8MrJKa7VYuJSdkMgV4GkJA6e4UnsoGWoYrWsikt5mhNg');

    const poaStateKey = getUtf8Codec().encode(poaName);

    const [proposalStoragePDA] = await getProgramDerivedAddress({
      seeds: [
        'STORAGE',
        poaStateKey,
        getUtf8Codec().encode(proposalStorageKey),
        getU64Codec({ endian: Endian.Little }).encode(storageSpace)
      ],
      programAddress: SOLANA_THRESHOLD_SIGNATURE_PROGRAM_PROGRAM_ADDRESS,
    });

    const [poaStatePDA] = await getProgramDerivedAddress({
      programAddress: SOLANA_THRESHOLD_SIGNATURE_PROGRAM_PROGRAM_ADDRESS,
      seeds: ['STATE', poaStateKey],
    });

    const res = {
      proposalInstructions: [
        {
          programAddress: ORACLE_PROGRAM_PROGRAM_ADDRESS,
          accounts: [
            {
              address: proposalStoragePDA,
              role: AccountRole.WRITABLE,
            },
            {
              address: poaStatePDA,
              role: AccountRole.READONLY,
            },
            {
              address: address('Sysvar1nstructions1111111111111111111111111'),
              role: AccountRole.READONLY,
            },
            {
              address: ORACLE_PROGRAM_PROGRAM_ADDRESS,
              role: AccountRole.READONLY,
            },
          ],
          data: getBase58Codec().decode(
            getCheckOracleInstructionDataCodec().encode({ poaStateKey }),
          ),
        },
      ],
      storagePayload: {
        encoding: 'utf-8',
        data: String(Date.now()),
      },
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
