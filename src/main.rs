use lwk_signer::SwSigner;
use lwk_wollet::{BlockchainBackend, ElectrumClient, EncryptedFsPersister, Wollet, WolletDescriptor};
use lwk_wollet::ElementsNetwork;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    let electrum_url = "blockstream.info:465".to_string();
    let mnemonic = "bacon bacon bacon bacon bacon bacon bacon bacon bacon bacon bacon bacon bacon bacon bacon bacon bacon bacon bacon bacon bacon bacon bacon bacon".to_string();
    let network: ElementsNetwork = ElementsNetwork::LiquidTestnet;

    let el_network: ElementsNetwork = network.into();
    let is_mainnet = el_network == ElementsNetwork::Liquid;
    let signer: SwSigner = SwSigner::new(&mnemonic, is_mainnet)?.into();
    let script_variant = lwk_common::Singlesig::Wpkh;
    let blinding_variant = lwk_common::DescriptorBlindingKey::Slip77;
    let desc_str =
        lwk_common::singlesig_desc(&signer, script_variant, blinding_variant, is_mainnet)?;
    println!("{:?}", desc_str);
    let descriptor = WolletDescriptor::from_str(&desc_str)?;

    let dbpath = "/tmp/lwk".to_string();
    let mut wollet = Wollet::new(
        network,
        EncryptedFsPersister::new(dbpath, network.into(), &descriptor)?,
        &desc_str,
    )?;

    let mut electrum_client: ElectrumClient = ElectrumClient::new(&lwk_wollet::ElectrumUrl::Tls(electrum_url, false))?;
    let update: Option<lwk_wollet::Update> = electrum_client.full_scan(&wollet)?;
    let _ = wollet.apply_update(update.unwrap());

    let txs = wollet.transactions()?;

    for tx in txs {
        println!("Tx: {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?}", tx.balance, tx.timestamp.unwrap(), tx.fee, tx.height, tx.type_, tx.tx.version, tx.tx.lock_time, tx.tx.vsize(), tx.tx.size(), tx.tx.weight());
        println!(" - Inputs:");
        for (i, input) in tx.inputs.iter().enumerate() {
            if let Some(tx_in) = input.as_ref() {
                println!("    - {:?} {:?} {:?}", i, tx_in.outpoint.txid, tx_in.outpoint.vout);
                println!("    - {:?} {:?} {:?} {:?} {:?}", i, tx_in.ext_int, tx_in.height, tx_in.wildcard_index, tx_in.script_pubkey.to_string());
                println!("    - {:?} {:?} {:?}", i, tx_in.unblinded.asset.to_string(), tx_in.unblinded.value);
            }
        }

        println!(" - Outputs:");
        for (i, output) in tx.outputs.iter().enumerate() {
            if let Some(tx_out) = output.as_ref() {
                println!("    - {:?} {:?} {:?} {:?}", i, tx_out.script_pubkey, tx_out.unblinded.asset, tx_out.unblinded.value);
            }
        }
        }

    Ok(())
}
