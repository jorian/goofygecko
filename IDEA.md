# Verus NFT implementation
(working document)

If the last 2 years have shown anything, it's that NFTs have made a great push into the market. You only have to look at how Bored Apes or Cryptopunks have performed (and are performing) to see that NFTs are not going to go away anytime soon.

When starting this project, I wasn't sure whether NFTs are just a fad or not. After fleshing out the details of NFTs on Verus I'm still not entirely convinced, but I don't see them going away either. However, NFTs are not the sole reason I started this project. I'm interested in the Verus platform for a while and wanted to learn more about it, so why not do so by building something cool.

NFTs already exist on plenty of other platforms, why try the same thing on a different platform?

- Use Verus Identities (and sub-identities)
- Have sovereignty over NFTs, and get rid of storing an NFT on a simple address / account which are vulnerable for Metamask hacks.
- Do more – Showcase what's possible with the Verus Identities combined with NFT.
    - Login with your NFT
    - Showcase what the powers of vdxfids are
    - Vault your NFT (in the protocol, not at third parties)
    - < probably lots more that i can't think of right now >

Put simply: showing that the same and more can be done on a better platform.

This document describes the specification of using Verus Identities to make NFTs. As a gentle gesture to the Komodo platform, I chose geckos to be the main theme of this NFT series. Goofy Geckos, because, well, they are goofy when you see them. (A gecko is like a mini komodo, and Verus originated from Komodo)

Not only will this be used as a showcase, but also a form of documentation in which I want to compare NFTs on their different platforms. In order to make it understandable, a comparison will be made using the widely used ERC-721 standard to have a known starting point. Later on, I plan to write several blog posts outlining the decisions that were made and explain how Verus tech is used.

The goal is not to try to copy the ERC-721 capabilities over to Verus, but to understand how NFTs are created on Ethereum, how we can use NFTs on Verus and what more we can do with NFTs controlled with Verus IDs.

Note: this is mostly an exploration of NFTs built with Verus IDs. Nothing that is done in this project is by any means a standard or an attempt to achieve one. While it is desirable to use arrive at a standard at some point, this project is not trying to create or enforce one.

## The idea

A new Discord will be made with the purpose of having a community around Goofy Geckos. A Gecko, because of the legacy Verus has to the Komodo Platform. 
Every new member of the Goofy Gecko Discord will automatically get a Goofy Gecko NFT. The NFT is generated based on the Discord `user_id` which is a 12 digit number. With this number, traits and characteristics are chosen and put together into an image of a Goofy Gecko. 

![Test Goofy Gecko](https://nan3j6kjviaqz6lrbryrdxwz4t37n6v7y35kudcrg7qmu2smliwa.arweave.net/aBu0-UmqAQz5cQxxEd7Z5Pf2-r_G-qoMUTfgympMWiw)

All NFTs will have a rarity index, based on the presence of rare traits and characteristics.

Some of the traits and characteristics:
-	base colors and special versions of the gecko
-	eye variations
	-	sunglasses
-	tongue
-	nose ring
-	necklace
-	unicorn
-	hat / sombrero / cap
-	pipe / cigarette / cigar

The image and the metadata are uploaded to Arweave and tagged with the Verus identity that is controlling them. This way, instead of having an Account on Ethereum that controls the NFT, a Verus ID controlling the NFT.

The identity is created through a centralized currency GoofyGecko@. All NFTs will be controlled by a Goofy Gecko sub-ID that is issued from the GoofyGecko@ currency.

Using the on-chain marketplace (released on a later date) users can buy and sell Goofy Geckos, or simply gift them to others.

## NFTs on Ethereum

To understand the creation of NFTs on Verus, let's first skim over ERC-721 standard NFTs to get a proper understanding of what NFTs are, or what they are understood to be.

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

The ability to transfer NFTs on Ethereum is done by the contract creator defining the rules of transfer. Normally this means that only the current owner of the NFT can move ownership. The ERC721 interface functions for transferring ownership are pre-defined by the standard, but can be overwritten by contract writers to add extra capabilities.

Accounts on Ethereum are secured only by their public / private key pair, usually stored in browser-extensions like Metamask.
< could go into detail why Accounts on Ethereum are not mega safe >

### ERC721 NFT

So what makes an ERC-721 token an NFT?

"While an NFT is designed to represent the original asset on the blockchain, the NFT itself is seen as a separate entity from any content it contains." (https://nftnow.com/guides/what-is-nft-meaning/)

It's comparable to when you offer something as "a token of appreciation". The appreciation is mine, and I offer something that references that appreciation. A golden necklace given to someone will always remind that someone of the person who gifted the necklace. 

Therefore, the asset itself and the NFT are two separate things.

#### Digital asset vs NFT


When we look at a Bored Ape as an example (the Bored Ape Yacht Club contract was written using OpenZeppelin's standard), we see the image is stored on IPFS and has a URI of `QmPCDfVGgU214chp2sfdT1MjabkUXomqrREwy6ZiSuahGj`, which is an IPFS hash and can be located through a IPFS gateway; https://ipfs.filebase.io/ipfs/QmPCDfVGgU214chp2sfdT1MjabkUXomqrREwy6ZiSuahGj

That is all there is to the image. It exists and is hosted on IPFS, without any markers it is being used as an NFT.

The metadata of this Bored Ape is stored somewhere else: https://ipfs.filebase.io/ipfs/QmTshKkZPL5Vrac4bLMimq4YgYzRhce7yDp2tiUsjKxuWB

`QmTshKkZPL5Vrac4bLMimq4YgYzRhce7yDp2tiUsjKxuWB` is the tokenURI that is set in the ERC721 standard, which also serves as the tokenID. It is the unique identifier that is used to make it the distinguishable part of an NFT. 

This metadata file contains the reference to the image: 
```json
{
  "image": "ipfs://QmPCDfVGgU214chp2sfdT1MjabkUXomqrREwy6ZiSuahGj",
  "attributes": []
}
```

## NFTs on Verus (concept)

Let's first try and map the ERC721 way of creating NFTs to Verus using Verus ID. _(The technical implementation details are found in [the next chapter](#nfts-on-verus-technical-implementation))_

A centralized PBaaS currency is created on the Verus blockchain, that allows the owner of the GoofyGecko@ ID to mint new sub-IDs. These sub-IDs will control the NFTs that are minted with every new member of the Discord server.

Recall that in the ERC-721 NFT contract on Ethereum, the tokenID is the tokenURI of the metadata.json (as hosted on IPFS), which contains a link to the image.

Because Ethereum uses IPFS (which is a file system) for storing the image, there is no possibility to attach any form of metadata to this image upload. When storing the image on Arweave (which is a blockchain and not a filesystem), we can attach metadata to this image upload. The end result is still the same; the image is still just the image, but the blockchain enforced way of putting the image there, allows us to add metadata to that upload which is enforced by the blockchain.

We can use this to our advantage and attach the name of the controlling ID to the image upload, defining who controls ownership of this image that is a NFT.

The image and nft.json are uploaded to Arweave with in their tags the vdxfid of what they are and to which ID they belong. The nft.json will also contain a link to the asset, among other attributes (all denoted in their vdxfids). Then, in the contentmap of the sub-ID, a key-value pair will be set that indicates the tokenURI, being the transaction hash of the arweave upload of the nft.json. 

Adding the transaction id of the metadata to the contentmap of the goofygecko sub-ID and adding the sub-ID name to the transaction of the image upload are extra steps. Compared to the ERC721 NFTs, they are not necessary; the image does not necessarily have to have an indication that it is an NFT. Also, given an ERC-721 address, there is nothing that indicates, from that address point-of-view, that it owns an NFT in the context of a ERC721 contract implementation.

### NFT ownership in Discord
Discord allows for bots to make extra functionalities in the backend, like connecting to a Verus daemon to Discord. This gives NFT owners in this Discord the ability to interact with the Verus daemon through a set of predefined bot commands, to do things like selling their NFT or gift them to someone.

How is ownership tracked in the Discord bot? From the Discord bot's point of view, there are 2 types of access to the identity: write and read access.

#### Write-access
The bot has write-access as long as there is a Verus R-address in the primary-addresses of the identity and there is only 1 signature required to make changes to the identity. In this state, the Discord bot has full write-access; it can update the content-map or transfer ownership to a different discord user.

Write access comes in 2 forms: 
- The Discord bot has sole ownership of the NFT
- The user adds his own R-address to the primary-addresses, making it a co-owned NFT.

#### Read-access
Without the Verus R-address in the primary-addresses of the ID, there is not much the bot can do with this ID. Read-access means that the bot can only use public information from the Verus blockchain and Arweave. It can not use the bot's functions such as updating the contentmap and transferring ownership.

#### Deposit / withdraw
With the ability of Discord users to attach their own controlling address to the goofygecko sub-ID, the functionality of depositing or withdrawing a sub-ID can be mimicked. 

Depositing an NFT to the Discord bot happens by getting an address from the Discord bot that is mapped to your user-id. Adding this address to the goofygecko sub-ID 'deposits' the sub-ID to the Discord bot, after which all the functionalities of the Discord bot are available.

In the same manner, withdrawing is doing the opposite of depositing. The user simply updates the sub-ID to only have an address that is owned by him. This way, the Discord bot does not have write-access anymore.

### Marketplace

As long as the Discord bot has write-access to the sub-ID, the sub-ID can make use of the Goofy Gecko Marketplace. It basically is a wrapper around the existing on-chain marketplace and allows Discord users to put their NFT for sale or to bid on other existing NFTs. When a deal is made, the Discord bot uses the on-chain marketplace functions to transfer ownership of the NFT to the new user. 

One could say this looks like an Opensea implementation, only in this case, you always keep control over the NFT.

It is important to note that at all times, the rightful owner of the NFT remains in full control; the revoke and recover can be set to an identity that is owned by the discord user, or 2 primary addresses controlling the ID can be set, one of which is the discord user and the other the Discord bot. 

However, in the end, the Discord user still needs to place some trust in the Discord bot not to run off with their NFTs, just like a user has to trust Opensea not to steal their deposited NFTs. Verus Vault could be used to mitigate this attack vector, by allowing users to place a unlock timer, such that updates or withdraws of the NFT need to await a certain amount of time. This would give the user enough confidence to trust the bot with the NFT.

## NFTs on Verus (technical implementation)
todo:
- reasoning behind the goofygecko currency definition (options: 34, proofprotocol: 2)
- slash commands in Discord

There are three parts to the NFT that is minted using Verus Identities; 
- The digital asset that is being tokenized,
- The JSON file containing the token (nft.json), being the overall token itself with additional metadata;
- The Verus ID

### Digital asset
The digital asset is uploaded to Arweave. As Arweave allows tags to be uploaded with the file permanently, we attach the following key-value tags to the asset file:

```json
[
  {
    "name": "<vdxfid of goofygecko.vrsctest::nft.identity.name>", 
    "value": "1.goofygecko@" 
  },
  {
    "name": "<vdxfid of vrsctest::nft",
    "value": "<vdxfid of goofygecko.vrsctest::nft.image>"
  },
  {
    "name": "Content-Type", 
    "value": "image/png" 
  }
]
```

Where the 1 in front of goofygecko is the number of the Goofy Gecko NFT series. This is used for querying Arweave; if an application wants to know which image belongs to Goofy Gecko #1, it can query for its tags.

Uploads to Arweave are done using an Arweave address that is tied to the GoofyGecko@ identity, proving that only uploads from this address are the real deal. The following GraphQL query shows how to only get real goofy geckos:

```graphql
query {
  transactions(
    owners:["YBJ4lMG5wTETx3_Uh4q3hLZdfUJS7qcxSVdG6vwBkV8"],
    tags: [
        {
            name: "<vdxfid of goofygecko.vrsctest::nft.identity.name>",
            values: ["1.goofygecko@"]
        },
        {
            name: "<vdxfid of vrsctest::nft>",
            values: ["<vdxfid of goofygecko.vrsctest::nft.image>"]
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

A signed message, signed with the GoofyGecko@ identity, saying that `<arweave address> is owned by goofygecko@` and using this signature somewhere public for anyone to verify that it was signed by the goofygecko@ identity, proves that GoofyGecko@ owns the Arweave address.

#### NFT.json (JSON file)
After uploading the digital asset to Arweave, the location of the asset is known. So, we can link to it and put this link in the metadata file (NFT.json). 

This metadata file is also uploaded to Arweave as it is this file that makes it the token of the digital asset. The NFT.json has the following contents:

- vrsc::nft.art.image.arweavetx (link to image on arweave)
- vrsc::nft.art.name
- vrsc::nft.art.rarity
- vrsc::nft.art.attributes.base
- vrsc::nft.art.attributes.background
- vrsc::nft.art.attributes.hat
- vrsc::nft.art.attributes.eye
- vrsc::nft.art.attributes.tongue
- vrsc::nft.art.attributes.necklace
- vrsc::nft.art.license.arweavetx
- vrsc::nft.art.license.signatures.minter
(more to be determined)

The NFT.json is uploaded to Arweave after which the transaction hash is hex-encoded. The upload will contain a tag, just like the image, only this time it states that it is the nft.json file: 

```json
[
  {
    "name": "<vdxfid of goofygecko.vrsctest::nft.identity.name>", 
    "value": "1.goofygecko@" 
  },
  {
    "name": "<vdxfid of vrsctest::nft",
    "value": "<vdxfid of goofygecko.vrsctest::nft.json>"
  },
  {
    "name": "Content-Type", 
    "value": "application/json" 
  }
]
```

#### License
A license, cryptographically signed by the minter, waiving all rights pertaining to the digital asset and giving the identity that owns the NFT full ownership and rights. This will be uploaded to Arweave and linked to in the NFT metadata.

#### Ownership
Verus Identities can be controlled by 1 or more VRSC transparent addresses (starts with R). When there is more than 1 controlling address, the minimum number of required signatures to spend from the identity can be set to a number higher than 1, to indicate a multi-signature address:

This is useful in the context of the Discord bot, as we can define co-ownership of the Goofy Gecko. 

When the NFT is initially created, there is only 1 primary controlling address. The address is owned by the Discord bot and is mapped to the user_id. The Discord user then has the ability to add an extra address, controlled by him, giving the identity 2 addresses. This gives the user the ability to always be able to control the NFT he owns, and gives the discord bot the ability to move ownership as well (through a Discord marketplace in a later version).

It is also possible to completely withdraw the NFT from the bot, by setting the primary address to 1 address, controlled by the Discord user. The Discord bot will not have any more power over the identity, so some functionality in the context of the Discord bot will be lost. 

A subid issued from the GoofyGecko@ id will always have the ability to be deposited into the Discord bot, and regain all functionalities that the Discord bot offers.

Ultimately, this means that the sub-ID name _is_ the tokenID. `1.goofygecko@` is the identifier for the token, since this sub-ID is used to denote who or what owns the NFT. This is contrary to ERC-721, where it is a mapping of Ethereum acoount address to the tokenID. In Verus' case, the metadata will always be linked to the Verus ID, and this ID is then bought or sold using the on-chain marketplace.


#### Minting account
The Goofy Gecko sub-ID that manages ownership of the NFT is minted from the GoofyGecko@ currency. Only the owner of the GoofyGecko@ id will be able to create new subIDs.

This is similar to what happens in ERC-721, where only the contract owner can mint new tokens. The address of the contract owner is also there to verify that the NFT is part of the correct NFT series and is not a counterfeit NFT minted by a different contract.

## Comparison

#### Decentralized Apps
The greatest difference between minting NFTs on Ethereum vs Verus is that the process of minting an NFT is blockchain enforced on Ethereum. The back end logic is written in a smart contract and then deployed on the Ethereum blockchain. 

This is not the case with Verus. Where Ethereum's process is fixed inside the blockchain layer, Verus has to do it on the application layer, as the Verus protocol doesn't allow for customized contracts, therefore making it susceptible to intermediary changes, as the application logic is run on centralized servers maintained by some development team.

For example, several tutorials implementing ERC-721 use Chainlink to have provably fair random number generated values inside a blockchain. Having this blockchain-enforced gives people that interact with this contract the confidence that this contract will always be executed in the way that it exists on the blockchain and that random numbers generated within this contract are provably random.

With the idea outlined in this document, people will have to trust the way the application is designed will stay the same through time. Something they won't have to do when interacting with smart contracts on Ethereum.

#### Minting
In an ERC-721 contract, only the contract owner can mint new NFTs and send them to new owners. Similarly, only the owner of the ID that issues sub-IDs can issue sub-IDs and 'send' them to new owners. I use send in quotes, because what really happens is that the primary controlling addresses in the sub-ID are set to an address that is controlled by the Discord Bot, mapped to the user_id in the Discord bot. 
In other words, the minter is the ID that represents the series. In the PoC this will be the geckotest@ ID. The owner of this ID can only issue new sub-IDs, being the NFTs.

#### Tokenization
Assets are tokenized on ERC-721 by adhering to the ERC-721 standard of:
A unique name field;
Tracking ownership

Just like every token in an ERC-721 is unique because of the standard, sub-IDs issued from a parent ID are unique, simply because of their name. 

Ownership of an NFT is defined, again, using the standard that the ERC-721 provides. The contract writer has to adhere to these standards. Verus IDs already have ownership management in the protocol on which they function, by adding or removing controlling addresses, the ability to Vault an ID, or add recovery and revoke identities to restore access in case access is lost.

#### Ownership
Calling back to the EIP-721 improvement proposal, which states: "NFTs are distinguishable and you must track the ownership of each one separately.", we see that ERC721 NFT ownership is tracked using Ethereum accounts, which is (simplified) a private/public key pair. With Verus, identities are used to track ownership, which, arguably, is a better choice when it comes to recovery of NFTs.
ERC721 NFTs change ownership through the implementation of the ERC721 contract. Each individual ERC721 has its own implementation of the standard, which can cause bugs because of a faulty implementation. Verus NFTs don't require writing contracts; all you need is an identity which has implicit ownership tracking without having to write code. 

#### Querying
Recall that the ERC721 standard implements some functions that allow other application to query the NFT for information. How does Verus using their vdxf notation achieve the same thing?

When implementing this ERC721 interface, the contract should return a tokenURI pointing to the location of the metadata, normally through a function called: tokenURI().  We can achieve the same mechanics through the ID contentmap, where we can define a key using Verus' VDXF notation: `nft1.goofygecko.vrsctest::nft.tokenuri` and its value the CID of the metadata file (or NFT.json)
