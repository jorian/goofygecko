# VerusNFT

A proof-of-concept to showcase the possibilities of NFTs with Verus IDs.

## Overview

A Discord server that is its own community, where every new entrant gets a free NFT.

-	Discord community with NFTs and trade bot
-	Image generated with characteristics based on combination of randomness and user_id
-	Image + Metadata stored on Arweave
-	NFT ownership managed with Verus ID

## Idea

### Introduction 

A new Discord will be made with the purpose of having a community around Geckos. The Gecko is chosen because of the legacy Verus has to the Komodo Platform.  
Every new member of the Verus NFT Discord will automatically own a Gecko NFT. The NFT is generated based on the Discord `user_id` which is a 12 digit number. With this number and a bit of randomness, traits and characteristics are chosen and put together into an image of a Gecko. 

Traits and characteristics (not yet final)
-	20 base colors of the gecko
-	4 eye variations
	-	sunglasses
-	Tongue
-	Nose ring
-	Verus necklace
-	Unicorn
-	hat / sombrero / cap
-	pipe / cigarette / cigar

Among the first 50 users, 3 golden geckos will be minted

All NFTs will have a rarity index, based on the presence of rare traits and characteristics.

### Image creation

All the traits and characteristics are layers in an image. First the background color is set, then the base color of the gecko is added on top as a layer, then the eyes, etc. 

The selection of these traits and characteristics will be done using the unique Discord user_id, sprinkled with a bit of randomness. From the result, the traits and characteristics will be picked and merged into one image.

### Storage

Arweave is chosen to store the image. As the image will be 8-bit art, the image size will be relatively small. 
The Discord bot will be the creator of the image + metadata files and thus the Arweave address that is used to upload the image + metadata will be a tool to identify true Geckos.

### NFT Ownership (Verus IDs)

Verus IDs are used to manage ownership. A currency on Verus will be used / created to create the identities that are needed for the NFTs. 

1.	When an image is generated, it gets uploaded to Arweave. 
2.	The resulting transaction hash is hex-encoded. This hex value will be used as the ID name on a still-to-be-created Verus currency. 
3.	A message is signed with the newly created ID that says: `image_transaction_hash is owned by <id>@`
4.	A `metadata.json` file is created with the following contents:
	-	the signed message
	-	hash of image
	-	owner of image (Verus ID)
	-	url of image
	-	attributes (traits, characteristics, rarity)
5.	The `metadata.json` file is uploaded to Arweave and gets a tag: 
	-	`verusid`: `<id>@` (or maybe make use of vdxf keys here, not sure how this will work)
6.	The content-map of the Verus ID will be updated:
	-	key: hash160 of `<id>.vrsctest::discordbot.metadata`
	-	value: hex encoded value of transaction hash of `metadata.json` upload to Arweave.

(maybe it's useful to generate a SHA256 hash of the image file and use that as identifier for the id. This way, we can add tags to the image upload on Arweave too, which can directly link to the Verus ID)

(A question about this: the owner of the ID can erase the content map. This is not really desirable in the case of NFTs)

Arweave has query capabilities where a query can be done to find transactions using a filter on tag values. By putting the VerusId in the tag of the metadata.json transaction, we can find the metadata that belongs to the Verus ID easily; the name of the ID is the tag on Arweave.

Possibility exists to have the NFT be co-owned by both Bot and User. The User has to give his address, then the bot will update the ID's primary addresses with this new address, so that there are 2 addresses with minimumsignatures of 1.  
This can mimic the concept of deposit and withdrawal: depositing an NFT to the Discord Bot for showcase or trading just requires the owner to add the Discord bot address to his ID. 

-	'withdraw' the NFT by changing the primary address of the identity
-	'deposit' NFT by adding an address (specific to the user) to the discord bot and update identity.

### Marketplace

Buy or sell using on-chain marketplace, interfaced to Discord through a bot
(to be described)

## Future

-	Let geckos mate! 2 eggs will be conceived out of which new Geckos will be born (minted).
-	Make a webpage to showcase geckos