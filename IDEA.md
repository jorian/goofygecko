# Verus NFT implementation
(working document)

NFTs are hot. If the last 2 years have shown anything, it's that NFTs have made a great push into the market. You only have to look at how Bored Apes or Cryptopunks have performed (and are performing) to see that NFTs are not going to go away anytime soon.

When starting this project, I wasn't sure whether NFTs are just a fad or not. After fleshing out the details of NFTs on Verus I'm still not entirely convinced, but I don't see them going away either. However, NFTs are not the sole reason I started this project. I'm interested in the Verus platform for a while and wanted to learn more about it, so why not do so by building something cool.

NFTs already exist on plenty of other platforms, why try the same thing on a different platform?

- Use Verus Identities (and sub-identities)
- Have sovereignty over NFTs, and get rid of storing an NFT on a simple address / account which are vulnerable for Metamask hacks.
- Do more – Showcase what's possible with the Verus Identities combined with NFT.
    - Login with your NFT
    - Showcase what the powers of vdxfids are
    - < probably lots more that i can't think of right now >

Put simply: showing that the same and more can be done on a better platform.

This document describes the specification of using Verus Identities to make NFTs. As a gentle gesture to the Komodo platform, I chose geckos to be the main theme of this NFT series. Goofy Geckos, because, well, they are goofy when you see them.

Not only will this be used as a showcase, but also a form of documentation in which I want to compare NFTs on their different platforms. In order to make it understandable, a comparison will be made using the widely used ERC-721 standard to have a known starting point. 

The goal is not to try to copy the ERC-721 capabilities over to Verus, but to understand how NFTs are created on Ethereum, how we can use NFTs on Verus and what more we can do with NFTs controlled with Verus identities.

Note: this is mostly an exploration of Verus identities. Nothing that is done in this project is by any means a standard or an attempt to achieve one. While it is desirable to use such a standard, this project is not trying to enforce one.

## Goofy Geckos

A proof-of-concept to showcase the possibilities of NFTs with Verus IDs.

### Overview

A Discord server that is its own community, where every new member gets a free NFT.

- Discord community with NFTs and trade bot
- Image generated with characteristics based on combination of randomness and user_id
- Image + Metadata stored on Arweave
- NFT ownership managed with Verus ID

A new Discord will be made with the purpose of having a community around Geckos. The Gecko is chosen because of the legacy Verus has to the Komodo Platform.  
Every new member of the Verus NFT Discord will automatically own a Gecko NFT. The NFT is generated based on the Discord `user_id` which is a 12 digit number. With this number, traits and characteristics are chosen and put together into an image of a Gecko.

All NFTs will have a rarity index, based on the presence of rare traits and characteristics.

## NFTs on Ethereum
> "DeFi platforms still provide certain rights to developers to alter their smart contract codes to ensure “vulnerabilities are patched effectively without waiting for approval by a consensus of users,”" - [_source_](https://www.outlookindia.com/business/hackers-used-key-loopholes-to-steal-12-billion-worth-of-nfts-claims-report-news-220266)

To understand what we're creating on Verus, let's first skim over ERC-721 standard NFTs to get a proper understanding of what NFTs are, or what they are understood to be.

### ERC-721
The ERC-721 standard is based on the EIP-721 improvement proposal, which states: "NFTs are distinguishable and you must track the ownership of each one separately." The EIP-721 was made to be a standard amongst developers so that applications could be built on top of it.

### IERC-721
To be compliant with the ERC-721 standard, Solidity programmers must implement at least the IERC721 interface, which consists of the following functions:

(see also https://betterprogramming.pub/bored-ape-yacht-club-smart-contract-breakdown-6c254c774394)

- balanceOf(owner)
- ownerOf(tokenId)
- safeTransferFrom(from, to, tokenId)
- transferFrom(from, to, tokenId)
- approve(to, tokenId)
- getApproved(tokenId)
- setApprovalForAll(operator, _approved)
- isApprovedForAll(owner, operator)
- safeTransferFrom(from, to, tokenId, data)

Events are emitted in the contract when the following happens:

- Transfer(from, to, tokenId)
- Approval(owner, approved, tokenId)
- ApprovalForAll(owner, operator, approved)

#### IERC721 Metadata
This is the (extension) interface that allows the smart contract to be asked (queried) for details about the asset that the NFT represents.
- Name
- Symbol
- tokenURI

### Ownership
The ERC721 interface functions for transferring ownership are pre-defined by the standard, but can be overwritten by contract writers to add extra capabilities.

---

So what makes an ERC-721 token an NFT?

"While an NFT is designed to represent the original asset on the blockchain, the NFT itself is seen as a separate entity from any content it contains." (https://nftnow.com/guides/what-is-nft-meaning/)

#### Digital asset vs NFT
- URI (or URL) of digital asset in metadata.json, usually a IPFS hash
- Generated (unique) token
- A contract that creates and takes care of transferring the NFT to new owners

When we look at a Bored Ape as an example (the Bored Ape Yacht Club contract was written using OpenZeppelin's standard), we see the image is stored on IPFS and has a URI of `QmPCDfVGgU214chp2sfdT1MjabkUXomqrREwy6ZiSuahGj`, which is an IPFS hash and can be located through a IPFS gateway; https://ipfs.filebase.io/ipfs/QmPCDfVGgU214chp2sfdT1MjabkUXomqrREwy6ZiSuahGj

That is all there is to the image. It exists and is hosted on IPFS, without any markers it is being used as an NFT (although you could say that anyone hosting images on something like IPFS is likely to be an NFT).

The metadata of this Bored Ape is stored somewhere else: https://ipfs.filebase.io/ipfs/QmTshKkZPL5Vrac4bLMimq4YgYzRhce7yDp2tiUsjKxuWB

`QmTshKkZPL5Vrac4bLMimq4YgYzRhce7yDp2tiUsjKxuWB` is the tokenURI that is set in the ERC721 standard, which also serves as the tokenID. It is the unique identifier that is used to make it the distinguishable part of an NFT. 


### Other implementations
ERC-1155 is a newer proposal for NFTs that also supports fungible tokens; it basically combines ERC20 and ERC721. 

## NFTs on Verus

Let's first try and map the ERC721 way of creating NFTs to Verus using Verus ID.

<how an NFT is created using Verus identities, upload to Arweave and where tokenURI goes, referencing ETH process>

### Ownership
The ability to transfer NFTs on Ethereum is done by the contract creator defining the rules of transfer. Normally this means that only the current owner of the NFT can move ownership.

Accounts on Ethereum are secured only by their public / private key pair and have none of the 'extras' that Verus has to offer.

Transferring an NFT, where ownership is controlled by a Verus identity, is done with updating the primary-addresses that control the Verus identity. A primary address is also a public / private key-pair. 

An owner of a Verus identity can use Verus Vault, making it impossible to move ownership in the event of a hack, by locking the identity for a certain period of time, or until an unlock occurs. Granted, this is also possible by writing a Solidity contract, but in Verus, this is part of the protocol, does not require 3rd-party storage or vaulting, and holds true for every Verus ID. 

### Arweave
Arweave allows tags on their uploads, which can be used to query the digital asset. In order to link the image to the ID, we can use the vdxfid of the ID as a tag on the image, so that we can find the image using the tag.

```json 
{ 
    "name": "vdxfid", 
    "value": "iA1RUcMrN5ioEpAo6kqxMAuqu7vQK59bY2" 
}
```

This is a one way look-up from anywhere (but mainly Verus) to the image. The vdxfid above is a result of the following getvdxfid call:  
`./verus getvdxfid 1.goofygecko.vrsctest::nft.attributes.image`

This means that applications can use this translation to get the image of Goofy Gecko #1.

Recall that on Ethereum, metadata.json contains a link to the image and similarly, an IPFS hash of the metadata.json is stored within the tokenURI of the ERC-721 token. This is what is the token in Non-Fungible Token.

Because Ethereum uses IPFS for storing the image, there is no opportunity to attach any form of metadata to this image upload. When storing the image on Arweave (which is a blockchain and not a filesystem), we can attach metadata to this image upload. The end result is still the same; the image is still just the image, but the blockchain enforced way of putting the image there, allows us to add metadata to that upload that is blockchain enforced.

We can use this to our advantage and attach the name of the controlling ID to the image upload, defining who controls ownership of this image that is a NFT.

The image and nft.json are uploaded to Arweave with in their tags the vdxfid of what they are and to which ID they belong. The nft.json will also contain a link to the asset, among other attributes all denoted in their vdxfids. Then, in the contentmap of the sub-ID, a key-value pair will be set that indicates the tokenURI, being the transaction hash of the arweave upload of the nft.json. 

Because the contentmap of the ID contains a link to the nft.json that has the vdxfid of that same ID, we can prove that the ID owns the digital asset that is linked in the nft.json. Only the owner of the ID has the power to change the content map.

### Discord
How is ownership tracked in the Discord bot? From the Discord bot's point of view, there are 2 types of access to the identity: write and read access.

#### Write-access
The bot has write-access as long as there is a Verus R-address in the primary-addresses of the identity and there is only 1 signature required to make changes to the identity. In this state, the Discord bot has full write-access; it can update the content-map or transfer ownership to a different discord user.

Write access comes in 2 forms: 
The Discord bot has sole ownership of the NFT
The user adds his own R-address to the primary-addresses, making it a co-owned NFT.

#### Read-access
Without the Verus R-address in the primary-addresses of the ID, there is not much the bot can do with this ID. Read-access means that the bot can only use public information from the Verus blockchain and Arweave. It can not use the bot's functions such as updating the contentmap and transferring ownership. 

### Discord marketplace
TODO  
Should outline the way a NFT can be deposited and shown on the Discord server. The discord server can be used to buy or sell NFTs and withdraw them afterwards.
I want to make the point that at all times, the rightful owner of the NFT remains in full control; the revoke and recover can be set to an identity that is owned by the discord user, or that there are 2 primary addresses controlling the ID, one of which is the discord user and the other the Discord bot.

It's like an identity with a face.

## Technical implementation
Conceptually, there are three parts to the NFT that is minted using Verus Identities; 
- The digital asset that is being tokenized,
- The JSON file containing the token (nft.json), being the overall token itself with additional metadata;
- The Verus ID

### Digital asset
The digital asset is uploaded to Arweave. As Arweave allows tags to be uploaded with the file permanently, we attach the following key-value tags to the asset file:

```json
[
  {
    "name": "identity", 
    "value": "1.goofygecko@" 
  },
  {
    "name": "Content-Type", 
    "value": "image/png" 
  }
]
```

Where the 1 in front of goofygecko is the number of the Goofy Gecko NFT series. This is used for querying Arweave; if an application wants to know which image belongs to Goofy Gecko #1, it can query for its tags.

Uploads to Arweave are done using an Arweave address that is tied to the goofygecko@ identity, proving that only uploads from this address are the real deal. The following GraphQL query shows how to only get real geckos:

```graphql
query {
  transactions(
    owners:["YBJ4lMG5wTETx3_Uh4q3hLZdfUJS7qcxSVdG6vwBkV8"],
    tags: [
        {
            name: "identity",
            values: ["1.goofygecko@"]
        }
    ]
  )
  {
    edges {
        node {
            id
       }
     }
  }
}
```

( https://gql-guide.vercel.app/#owners, https://gql-guide.vercel.app/#transaction )

We can also sign a message with the goofygecko@ identity, saying that `<arweave address> is owned by goofygecko@` and using this signature somewhere public for anyone to verify that it was signed by the goofygecko@ identity.
