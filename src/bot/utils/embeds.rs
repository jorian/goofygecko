use serenity::builder::CreateEmbed;

use crate::nft::VerusNFT;

pub fn from_verusnft(e: &mut CreateEmbed, verus_nft: VerusNFT) -> &mut CreateEmbed {
    // Todo: let VerusNFT have a metadata variable.
    e.title(format!(
        "Introducing TEST Goofy Geckos TEST #{}",
        verus_nft.sequence
    ))
    .description(format!("**Rarity:** {}\n", verus_nft.rarity))
    .field(
        "Transaction",
        format!(
            "[view](https://v2.viewblock.io/arweave/tx/{})",
            verus_nft.uploaded_image_tx_hash.as_ref().unwrap()
        ),
        true,
    )
    .field(
        "Metadata",
        format!(
            "[view](https://v2.viewblock.io/arweave/tx/{})",
            verus_nft.uploaded_metadata_tx_hash.as_ref().unwrap()
        ),
        true,
    )
    .image(format!(
        "https://arweave.net/{}",
        &verus_nft.uploaded_image_tx_hash.as_ref().unwrap()
    ))
}
