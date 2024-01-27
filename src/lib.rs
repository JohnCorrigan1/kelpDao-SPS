mod abi;
mod pb;
use hex_literal::hex;
use pb::kelp_dao::{AssetDeposit, AssetDeposits};
use substreams::scalar::{BigDecimal, BigInt};
use substreams::store::{StoreAdd, StoreAddBigInt, StoreGet, StoreGetBigInt, StoreNew};
use substreams::Hex;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables;
use substreams_ethereum::pb::sf::ethereum::r#type::v2 as eth;

// kelp dao LRT deposits contract
const TRACKED_CONTRACT: [u8; 20] = hex!("036676389e48133B63a802f8635AD39E752D375D");

substreams_ethereum::init!();

#[substreams::handlers::map]
fn map_deposits(blk: eth::Block) -> Result<Option<AssetDeposits>, substreams::errors::Error> {
    let asset_deposits: Vec<_> = blk
        .events::<abi::kelp_deposits::events::AssetDeposit>(&[&TRACKED_CONTRACT])
        .map(|(event, log)| {
            let asset_name = get_asset_name(&Hex::encode(&event.asset));
            AssetDeposit {
                depositor: "0x".to_string() + &Hex::encode(&event.depositor),
                asset: "0x".to_string() + &Hex::encode(&event.asset),
                asset_name,
                deposit_amount: event.deposit_amount.to_string(),
                deposit_amount_readable: get_amount_eth(event.deposit_amount),
                rseth_mint_amount: event.rseth_mint_amount.to_string(),
                rseth_mint_amount_readable: get_amount_eth(event.rseth_mint_amount),
                referral_id: event.referral_id.to_string(),
                block_number: blk.number,
                trx: "0x".to_string() + &Hex::encode(&log.receipt.transaction.hash),
                timestamp: blk.timestamp().to_string(),
                ordinal: log.block_index() as u64,
            }
        })
        .collect();

    Ok(Some(AssetDeposits { asset_deposits }))
}

#[substreams::handlers::store]
fn store_deposits(deposits: AssetDeposits, s: StoreAddBigInt) {
    for deposit in deposits.asset_deposits {
        s.add(
            0,
            &deposit.depositor,
            BigInt::try_from(&deposit.rseth_mint_amount).unwrap(),
        );
        s.add(
            0,
            format!("{}:{}", deposit.asset_name, deposit.depositor),
            BigInt::try_from(deposit.deposit_amount).unwrap(),
        );
        substreams::log::info!("referral_id: {}", deposit.referral_id);
        if deposit.referral_id == "" {
            continue;
        }
        s.add(
            0,
            deposit.referral_id,
            BigInt::try_from(&deposit.rseth_mint_amount).unwrap(),
        );
    }
}

#[substreams::handlers::map]
fn graph_out(
    deposits: AssetDeposits,
    deposit_store: StoreGetBigInt,
) -> Result<EntityChanges, substreams::errors::Error> {
    let mut tables = Tables::new();

    for deposit in deposits.asset_deposits {
        let key = format!("{}-{}", deposit.depositor, deposit.timestamp);
        tables
            .create_row("AssetDeposit", key)
            .set("depositor", &deposit.depositor)
            .set("asset", deposit.asset)
            .set("assetName", deposit.asset_name)
            .set(
                "depositAmount",
                BigInt::try_from(deposit.deposit_amount).unwrap(),
            )
            .set_bigdecimal(
                "depositAmountReadable",
                &deposit.deposit_amount_readable.to_string(),
            )
            .set(
                "rsethMintAmount",
                BigInt::try_from(deposit.rseth_mint_amount).unwrap(),
            )
            .set_bigdecimal(
                "rsethMintAmountReadable",
                &deposit.rseth_mint_amount_readable.to_string(),
            )
            .set("referralId", &deposit.referral_id)
            .set("blockNumber", deposit.block_number)
            .set("trx", deposit.trx)
            .set("timestamp", deposit.timestamp);

        tables
            .update_row("Depositor", &deposit.depositor)
            .set(
                "rsEthMinted",
                deposit_store
                    .get_at(0, &deposit.depositor)
                    .unwrap_or_default(),
            )
            .set(
                "ETHx",
                deposit_store
                    .get_at(0, &format!("ETHx:{}", deposit.depositor))
                    .unwrap_or_default(),
            )
            .set(
                "sfrxETH",
                deposit_store
                    .get_at(0, &format!("sfrxETH:{}", deposit.depositor))
                    .unwrap_or_default(),
            )
            .set(
                "stETH",
                deposit_store
                    .get_at(0, &format!("stETH:{}", deposit.depositor))
                    .unwrap_or_default(),
            );

        tables.update_row("Referral", &deposit.referral_id).set(
            "rsEthMinted",
            deposit_store
                .get_at(0, &deposit.referral_id)
                .unwrap_or_default(),
        );
    }

    Ok(tables.to_entity_changes())
}

fn get_amount_eth(amount: BigInt) -> f64 {
    let amount: BigDecimal = amount / 1e18;
    let amount = amount.to_string().parse::<f64>().unwrap();
    (amount * 1000.0).round() / 1000.0
}

fn get_asset_name(asset: &str) -> String {
    match asset {
        "a35b1b31ce002fbf2058d22f30f95d405200a15b" => "ETHx".to_string(),
        "ac3E018457B222d93114458476f3E3416Abbe38F" => "sfrxETH".to_string(),
        "ae7ab96520de3a18e5e111b5eaab095312d7fe84" => "stETH".to_string(),
        _ => "unknown asset".to_string(),
    }
}
