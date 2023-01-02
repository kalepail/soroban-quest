import time

from stellar_sdk import Network, Keypair, TransactionBuilder
from stellar_sdk import xdr as stellar_xdr
from stellar_sdk.soroban import SorobanServer
from stellar_sdk.soroban.soroban_rpc import TransactionStatus
from stellar_sdk.soroban_types import InvokerSignature, Int128, Int64

secret = <SOURCE_ACCOUNT_SECRET>
rpc_server_url = "<futurenet-horizon-url/soroban/rpc>"
network_passphrase = "Test SDF Future Network ; October 2022"

contract_id = "d93f5c7bb0ebc4a9c8f727c5cebc4e41194d38257e1d0d910356b43bfc528813" # Native XLM token contract id

kp = Keypair.from_secret(secret)

soroban_server = SorobanServer(rpc_server_url)
source = soroban_server.load_account(kp.public_key)

tx = (
    TransactionBuilder(source, network_passphrase)
    .set_timeout(300)
    .append_invoke_contract_function_op(
        contract_id=contract_id,
        method="import",  # or 'export'
        parameters=[
            InvokerSignature(),  # Invoker
            Int128(0),  # Nonce
            Int64(1_000 * 10**7)  # amount, 1,000 tokens
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