import time

from stellar_sdk import Network, Keypair, TransactionBuilder
from stellar_sdk import xdr as stellar_xdr
from stellar_sdk.soroban import SorobanServer
from stellar_sdk.soroban.soroban_rpc import TransactionStatus

secret = <SOURCE_ACCOUNT_SECRET>
rpc_server_url = "<futurenet-horizon-url/soroban/rpc>"
network_passphrase = "Test SDF Future Network ; October 2022"

contract_id = "71c670db8b9d9dd3fa17d83bd98e4a9814f926121972774bd419fa402fe01dc7" # Native XLM token contract id

kp = Keypair.from_secret(secret)

soroban_server = SorobanServer(rpc_server_url)
source = soroban_server.load_account(kp.public_key)

def get_nonce() -> stellar_xdr.SCVal:
    tx = (
        TransactionBuilder(source, network_passphrase)
        .set_timeout(300)
        .append_invoke_contract_function_op(
            contract_id=contract_id,
            method="nonce",
            parameters=[
                stellar_xdr.SCVal(
                    stellar_xdr.SCValType.SCV_OBJECT,
                    obj=stellar_xdr.SCObject(
                        stellar_xdr.SCObjectType.SCO_VEC,
                        vec=stellar_xdr.SCVec(
                            sc_vec=[
                                stellar_xdr.SCVal(
                                    stellar_xdr.SCValType.SCV_SYMBOL,
                                    sym=stellar_xdr.SCSymbol("Account".encode()),
                                ),
                                stellar_xdr.SCVal(
                                    stellar_xdr.SCValType.SCV_OBJECT,
                                    obj=stellar_xdr.SCObject(
                                        stellar_xdr.SCObjectType.SCO_ACCOUNT_ID,
                                        account_id=kp.xdr_account_id(),
                                    ),
                                ),
                            ]
                        ),
                    ),
                ),
            ],
            source=kp.public_key,
        )
        .build()
    )

    simulate_transaction_data = soroban_server.simulate_transaction(tx)
    return stellar_xdr.SCVal.from_xdr(simulate_transaction_data.results[0].xdr)


nonce = get_nonce()

source = soroban_server.load_account(
    kp.public_key
) # refresh source account, because the current SDK will increment the sequence number by one after building a transaction

tx = (
    TransactionBuilder(source, network_passphrase)
    .set_timeout(300)
    .append_invoke_contract_function_op(
        contract_id=contract_id,
        method="import", # or 'export'
        parameters=[
            stellar_xdr.SCVal( # Invoker
                stellar_xdr.SCValType.SCV_OBJECT,
                obj=stellar_xdr.SCObject(
                    stellar_xdr.SCObjectType.SCO_VEC,
                    vec=stellar_xdr.SCVec(
                        sc_vec=[
                            stellar_xdr.SCVal(
                                stellar_xdr.SCValType.SCV_SYMBOL,
                                sym=stellar_xdr.SCSymbol("Invoker".encode()),
                            ),
                        ]
                    ),
                ),
            ),
            nonce, # Nonce
            stellar_xdr.SCVal( # amount
                stellar_xdr.SCValType.SCV_OBJECT,
                obj=stellar_xdr.SCObject(
                    stellar_xdr.SCObjectType.SCO_I64,
                    i64=stellar_xdr.Int64(1_000 * 10**7), # 1,000 tokens
                ),
            ),
        ],
        source=kp.public_key,
    )
    .build()
)

simulate_transaction_data = soroban_server.simulate_transaction(tx)
print(f"simulated transaction: {simulate_transaction_data}")

print(f"setting footprint and signing transaction...")
tx.set_footpoint(simulate_transaction_data.footprint)
tx.sign(kp)

send_transaction_data = soroban_server.send_transaction(tx)
print(f"sent transaction: {send_transaction_data}")

while True:
    print("waiting for transaction to be confirmed...")
    get_transaction_status_data = soroban_server.get_transaction_status(
        send_transaction_data.id
    )
    if get_transaction_status_data.status != TransactionStatus.PENDING:
        break
    time.sleep(3)
print(f"transaction status: {get_transaction_status_data}")

if get_transaction_status_data.status == TransactionStatus.SUCCESS:
    print(f"transaction result: {get_transaction_status_data.results}")
    result = stellar_xdr.SCVal.from_xdr(get_transaction_status_data.results[0].xdr) # type: ignore
    print(result)