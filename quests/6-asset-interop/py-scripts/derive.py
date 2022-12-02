import hashlib

from stellar_sdk import Asset
from stellar_sdk import xdr as stellar_xdr

def get_asset_contract_id(asset: Asset) -> str:
    """Get the contract id of the wrapped token contract."""
    data = stellar_xdr.HashIDPreimage(
        stellar_xdr.EnvelopeType.ENVELOPE_TYPE_CONTRACT_ID_FROM_ASSET,
        from_asset=asset.to_xdr_object(),
    )
    contract_id = hashlib.sha256(data.to_xdr_bytes()).hexdigest()
    return contract_id

if __name__ == '__main__':
    asset = Asset.native() # Asset("HELLO", "GBCXQUEPSEGIKXLYODHKMZD7YMTZ4IUY3BYPRZL4D5MSJZHHE7HG6RWR")
    print(f"Contract ID: {get_asset_contract_id(asset)}")