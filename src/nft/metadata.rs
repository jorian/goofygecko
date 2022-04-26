// The user_id is used as an input to the randomizer function.
// This function will be called when a new member joins the Discord. The Event `GuildMemberAdd` is triggered, after which
// this function is called.

pub fn generate(user_id: u64) /* -> NFTMetadata */
{
    /* generates a NFTMetadata struct containing:
    {
        name: string, // Gecko #1
        symbol: string,
        image: string, // filepath under ./generated
        external_url: option<string>, // populated after uploading the NFT to Arweave
        edition: int,
        attributes: [
            { "trait_type": string, "value": string },
            ...
        ]
    }
    */
}
