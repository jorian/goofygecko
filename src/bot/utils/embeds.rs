use serenity::builder::CreateEmbed;

use crate::nft::VerusNFT;

pub fn from_verusnft(e: &mut CreateEmbed, nft_builder: VerusNFT) -> &mut CreateEmbed {
    e.title(format!("Introducing testgecko #{}", nft_builder.sequence))
        .description(format!("**Rarity:** {}\n**Price:** {} VRSC", 23, 12))
        .field(
            "Transaction",
            format!(
                "[view](https://v2.viewblock.io/arweave/tx/{})",
                nft_builder.uploaded_image_tx_hash.as_ref().unwrap()
            ),
            true,
        )
        .field(
            "Metadata",
            format!(
                "[view](https://v2.viewblock.io/arweave/tx/{})",
                nft_builder.uploaded_metadata_tx_hash.as_ref().unwrap()
            ),
            true,
        )
        .image(format!(
            "https://arweave.net/{}",
            &nft_builder.uploaded_image_tx_hash.as_ref().unwrap()
        ))
}
