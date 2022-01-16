https://github.com/GabeDottl/Solana-Game-Server

# Mint Tardigrades (using Candy Machine(?))
https://docs.metaplex.com/candy-machine-v2/introduction (This took longer to find than I'd expect).
Features: Unpredictable mint index, whitelists, captcha integration, 

"Larger collections and hide-and-reveal drops: It is possible to create super large and hide-and-reveal drops by specifying a single hash, which is used by all mints. While the hash is the same across all mints, the name of each item is specified with a unique number, **allowing an off-chain process to later update the metadata with the actual item.**"
^All on-chain including initialization logic and system parameters?

https://hackmd.io/@levicook/HJcDneEWF
Done git clone git@github.com:metaplex-foundation/metaplex.git
npm install -g ts-node
done yarn install
; You know you're "bleeding edge" when you immediately start finding issues
; https://github.com/metaplex-foundation/metaplex/issues/1537

platform-level note: Could do a hybrid approach - generate & push contract logic per-app to create an 'app in a box' on-chain combined with shared on-chain infrastructure. Platform apps get cheap rail-access?

https://hackmd.io/@levicook/HJcDneEWF # Start here.

Collection created (`python3 generate_metaplex_metadata.py`)
https://docs.metaplex.com/candy-machine-v2/creating-candy-machine

ts-node ./metaplex/js/packages/cli/src/candy-machine-v2-cli.ts upload -e devnet -k ~/dotfiles/devnet.json -cp config.json -c example ./sample_data/build/

https://docs.metaplex.com/candy-machine-v2/creating-candy-machine


# Metaplex API (Rust crates): metaplex_token_metadata_test_client, metaplex_token_vault_test_client, mpl_auction, mpl_auction_house, mpl_gumdrop, mpl_metaplex, mpl_nft_packs, mpl_token_entangler, mpl_token_metadata, mpl_token_vault, nft_candy_machine
https://metaplex-foundation.github.io/metaplex-program-library/mpl_token_metadata/
# TODO: How to create cool Rust Crate docs: https://metaplex-foundation.github.io/metaplex-program-library/mpl_token_metadata/index.html?

ts-node ~/metaplex/js/packages/cli/src/candy-machine-v2-cli.ts upload -e devnet -k ~/dotfiles/solana_wallet//devnet.json -cp config.json -c example ./sample_data

Error: If spl-token-account or spl-token is set then sol-treasury-account cannot be set
    at getCandyMachineV2Config (/Users/gabedottl/code/SGS/metaplex/js/packages/cli/src/helpers/various.ts:95:13)
    at Command.<anonymous> (/Users/gabedottl/code/SGS/metaplex/js/packages/cli/src/candy-machine-v2-cli.ts:99:38)
s/"solTreasuryAccount": "rmJDUHfhfyVSBkPrX6AYDYLG5yh9oWk2Sb7k1tHCQwN",//g
https://issueexplorer.com/issue/metaplex-foundation/metaplex/798

https://docs.metaplex.com/community#RPC - Community RPC services

Ha - skipped this: https://docs.metaplex.com/candy-machine-v2/configuration (see config.json)
https://docs.civic.com/candy-machine-integration/adding-captcha-to-candy-machine-v2#terms-and-conditions - huh, civic.

RunNode RPC endpoint:
https://app.runnode.com/projects

## Current status
`
❯ ts-node ~/code/SGS/metaplex/js/packages/cli/src/candy-machine-v2-cli.ts upload -e devnet -k ~/dotfiles/config/solana-wallets/devnet.json -cp ~/code/SGS/config.json -c tmp-cache ~/code/SGS/sample_data/build -l DEBUG
setting the log value to: DEBUG
wallet public key: DDe72EQdVnNQMH812CNv2Ac7WZ83B5SbrtRWYjc26HXm
program id from anchor cndy3Z4yapfJBmL3ShUp5exZKqR3z33thTzeNMm2gRZ
Beginning the upload for 3 (img+json) pairs
started at: 1642262453827
initializing candy machine
Error deploying config to Solana network. RangeError: indeterminate span
    at Structure.getSpan (/Users/gabedottl/code/SGS/metaplex/js/node_modules/buffer-layout/lib/Layout.js:1221:13)
    at Structure.encode (/Users/gabedottl/code/SGS/metaplex/js/node_modules/buffer-layout/lib/Layout.js:1267:23)
    at InstructionCoder._encode (/Users/gabedottl/code/SGS/metaplex/js/packages/cli/node_modules/@project-serum/anchor/src/coder/instruction.ts:85:24)
    at InstructionCoder.encode (/Users/gabedottl/code/SGS/metaplex/js/packages/cli/node_modules/@project-serum/anchor/src/coder/instruction.ts:68:17)
    at /Users/gabedottl/code/SGS/metaplex/js/packages/cli/node_modules/@project-serum/anchor/src/program/namespace/index.ts:51:56
    at ix (/Users/gabedottl/code/SGS/metaplex/js/packages/cli/node_modules/@project-serum/anchor/src/program/namespace/instruction.ts:43:15)
    at txFn (/Users/gabedottl/code/SGS/metaplex/js/packages/cli/node_modules/@project-serum/anchor/src/program/namespace/transaction.ts:17:14)
    at Object.rpc [as initializeCandyMachine] (/Users/gabedottl/code/SGS/metaplex/js/packages/cli/node_modules/@project-serum/anchor/src/program/namespace/rpc.ts:16:18)
    at createCandyMachineV2 (/Users/gabedottl/code/SGS/metaplex/js/packages/cli/src/helpers/accounts.ts:149:35)
    at processTicksAndRejections (node:internal/process/task_queues:96:5)
upload was not successful, please re-run. RangeError: indeterminate span
    at Structure.getSpan (/Users/gabedottl/code/SGS/metaplex/js/node_modules/buffer-layout/lib/Layout.js:1221:13)
    at Structure.encode (/Users/gabedottl/code/SGS/metaplex/js/node_modules/buffer-layout/lib/Layout.js:1267:23)
    at InstructionCoder._encode (/Users/gabedottl/code/SGS/metaplex/js/packages/cli/node_modules/@project-serum/anchor/src/coder/instruction.ts:85:24)
    at InstructionCoder.encode (/Users/gabedottl/code/SGS/metaplex/js/packages/cli/node_modules/@project-serum/anchor/src/coder/instruction.ts:68:17)
    at /Users/gabedottl/code/SGS/metaplex/js/packages/cli/node_modules/@project-serum/anchor/src/program/namespace/index.ts:51:56
    at ix (/Users/gabedottl/code/SGS/metaplex/js/packages/cli/node_modules/@project-serum/anchor/src/program/namespace/instruction.ts:43:15)
    at txFn (/Users/gabedottl/code/SGS/metaplex/js/packages/cli/node_modules/@project-serum/anchor/src/program/namespace/transaction.ts:17:14)
    at Object.rpc [as initializeCandyMachine] (/Users/gabedottl/code/SGS/metaplex/js/packages/cli/node_modules/@project-serum/anchor/src/program/namespace/rpc.ts:16:18)
    at createCandyMachineV2 (/Users/gabedottl/code/SGS/metaplex/js/packages/cli/src/helpers/accounts.ts:149:35)
    at processTicksAndRejections (node:internal/process/task_queues:96:5)
Execution time: 0h:00m:07s sec
`
; Note: Using aws
; Pausing trying to get metaplex working on devnet. aws: https://s3.console.aws.amazon.com/s3/buckets/tardigrades-bucket?region=us-east-2&tab=properties
; TODO: https://github.com/ArweaveTeam/arweave

; Discord Metaplex Candy Machinev2 Discord subchannel!!! https://discord.com/channels/848060988636921856/898625218426339448
Shame it's hard to inline paste into markdown docs from Visual Studio Code

https://developers.cloudflare.com/workers/platform/sites/start-from-worker -- Here


## TODO: for SGS, https://spl.solana.com/token#example-wrapping-sol-in-a-token
## Miscellaneous
$ spl-token transfer $SGS 50 $SGSACCOUNT
Transfer 50 tokens
  Sender: 7UX2i7SucgLMQcfZ75s3VXmZZY4YRUyJN9X1RgfMoDUi
  Recipient: vines1vzrYbzLMRdu58ou5XTby4qAqVRLmqo36NKPTg
  Recipient associated token account: F59618aQB8r6asXeMcB9jWuY6NEx1VduT9yFo1GTi1ks

Signature: 5a3qbvoJQnTAxGPHCugibZTbSu7xuTgkxvF4EJupRjRXGgZZrnWFmKzfEzcqKF2ogCaF4QKVbAtuFx7xGwrDUcGd

Need to figure out how to generate metadata from filenames. See generate_metaplex_metadata.py.

Building anchor project:
https://project-serum.github.io/anchor/getting-started/installation.html#build-from-source-for-other-operating-systems

https://doc.rust-lang.org/book/title-page.html - review.
https://dev.to/dabit3/the-complete-guide-to-full-stack-solana-development-with-react-anchor-rust-and-phantom-3291 - deferred.

## References
Metaplex Issue Tracker
https://github.com/metaplex-foundation/metaplex/issues?q=is%3Aissue+is%3Aopen+

Excellent CMv2 walkthrough (1/10/22): https://twitter.com/marcelc63/status/1480570145193951234
https://github.com/kevinfaveri/solana-candy-factory
https://github.com/exiled-apes/candy-machine-mint
Bot to post to Discord when NFTs sell: https://github.com/milktoastlab/SolanaNFTBot
https://mkyong.com/web/how-to-pretty-print-json-output-in-curl/

# Breed minted tardigrades
## Current status
Can't mint w/CMv2? (paused).
❯ ts-node metaplex/js/packages/cli/src/candy-machine-v2-cli.ts mint_one_token \
    -e devnet \
    -k ~/dotfiles/config/solana-wallets/devnet.json \
    -c example

/Users/gabedottl/code/SGS/metaplex/js/packages/cli/src/candy-machine-v2-cli.ts:713
    const candyMachine = new PublicKey(cacheContent.program.candyMachine);
                                                    ^
TypeError: Cannot read properties of undefined (reading 'program')
    at Command.<anonymous> (/Users/gabedottl/code/SGS/metaplex/js/packages/cli/src/candy-machine-v2-cli.ts:713:53)
    at Command.listener [as _actionHandler] (/Users/gabedottl/code/SGS/metaplex/js/packages/cli/node_modules/commander/lib/command.js:488:17)
    at /Users/gabedottl/code/SGS/metaplex/js/packages/cli/node_modules/commander/lib/command.js:1227:65
    at Command._chainOrCall (/Users/gabedottl/code/SGS/metaplex/js/packages/cli/node_modules/commander/lib/command.js:1144:12)
    at Command._parseCommand (/Users/gabedottl/code/SGS/metaplex/js/packages/cli/node_modules/commander/lib/command.js:1227:27)
    at Command._dispatchSubcommand (/Users/gabedottl/code/SGS/metaplex/js/packages/cli/node_modules/commander/lib/command.js:1050:25)
    at Command._parseCommand (/Users/gabedottl/code/SGS/metaplex/js/packages/cli/node_modules/commander/lib/command.js:1193:19)
    at Command.parse (/Users/gabedottl/code/SGS/metaplex/js/packages/cli/node_modules/commander/lib/command.js:897:10)
    at Object.<anonymous> (/Users/gabedottl/code/SGS/metaplex/js/packages/cli/src/candy-machine-v2-cli.ts:992:9)
    at Module._compile (node:internal/modules/cjs/loader:1095:14)

https://docs.metaplex.com/candy-machine-v2/mint-tokens

Going to mint natively. 
https://github.com/ArweaveTeam/arweave
https://docs.solana.com/integrations/exchange#token-mints
https://www.youtube.com/watch?time_continue=169&v=35RO0lAEIxE&feature=emb_logo
The docs:
https://docs.metaplex.com/create-cand...

Metaplex JS SDK
https://docs.metaplex.com/sdk/js/getting-started

https://github.com/metaplex-foundation/metaplex-program-library

Metaplex is a protocol built on top of Solana that allows:

Creating/Minting Non-Fungible Tokens;
Starting A variety of auctions for primary/secondary sales;
and Visualizing NFTs in a standard way across wallets and applications.

Metaplex is comprised of two core components: an on-chain program, and a self-hosted front-end web3 application.
https://github.com/metaplex-foundation/metaplex
TODO: https://docs.metaplex.com/security-policy


```
# NFT Auction e2e

1 allocate space for a mint account and a token account
2 spl-token JS SDK createMint and createTokenAccount commands to create a new mint and a new token account of that mint. You then use the mintTo command to mint a single token to that token account.
3 Token Metadata's create_metadata_account endpoint, naming it the "Bob's Cool NFT" mint with symbol "BOB" and you pass in a link to a picture of your Uncle Bob for the URI. This command creates a Metadata account with a PDA seed of ['metadata', metadata_program_id, your_mint_id] relative to the metadata_program_id.

4 If you, need to turn this NFT into a Master Edition:
4.5 create_master_edition endpoint on the Token Metadata contract which takes minting authority away from you, and creates a new Master Edition pda that contains information about how large a supply you want to have.

Limited Edition prints! Let's say we want to auction off three such prints.

5 create a token vault using the init_vault endpoint of the token vault contract. 
6 store our master edition token in it by adding it to the vault using the add_token_to_inactive_vault endpoint. This will create a safety deposit box in the vault that contains the the token.
7 call the activate_vault command which Activates the vault, locking everything inside.

Now that your token account has a bonafide NFT Master Edition in it, we can run an auction where we auction off 

Run combine_vault (opens it to allow the current authority to optionally withdraw the tokens inside it)
Note: Auction Manager can only work with vaults in this state, which is why we have to go through the Activation phase

create the auction (via the create_auction command on the Auction contract), unstarted, with the 'resource' being this vault

Now have an auction and a vault, call init_auction_manager on Metaplex contract with both accounts (+others) to create an AuctionManager, which ties them both together.
Note: init_auction_manager takes a special struct called AuctionManagerSettings that allows one to specify how many winners there are and what winners get which items from which safety deposit box
Note: AuctionManager is in an invalidated state; need to validate it by validating that the safety deposit boxes we provided to it in the vault are actually what we said are in them when we provided the AuctionManager with it's settings struct.

Before validation, call set_authority on both vault and auction to change authority to the auction manager, so that it has control over both of those structs. This is a requirement for the validation phase and the rest of the contract lifecycle. Now you no longer have control over your items.

We call the validate_safety_deposit_box endpoint on the Metaplex contract with the one safety deposit box in the vault, and the logic in this endpoint checks that there are exactly 3 printing tokens from the right mint in this box, matching the 3 printing tokens we promised it would have in our AuctionManagerSettings. Once we do this, and because this is the only safety deposit box in the vault, the AuctionManager is now validated.

We now call start_auction on the Metaplex contract, which, because the AuctionManager has authority over the Auction, calls start_auction on the Auction contract, and the auction begins!

Users can go and call place_bid on the Auction contract to place bids. When they do this, tokens of the token_mint type used by the auction are taken from the account they provide, tied to their main wallet, and stored in bidder pot accounts in the auction contract.

sIn order to update a bid, a user must first cancel the original bid, and then place a new bid.

Once the auction is over, a user can refund their bid if they did not win by calling cancel_bid again. Winners of the auction cannot cancel their bids.

The winner of a bid creates a mint with decimals 0, a token account with 1 token in it, and calls the redeem_printing_v2_bid endpoint on the Metaplex contract, all in a single transaction. This token is now officially a Limited Edition of the "Bob's Cool NFT" Master Edition NFT!

You, the auctioneer, visits /#/auction/id/billing and hit the settle button. This first iterates over all three bidders and for each wallet used, calls claim_bid on the Metaplex contract, which proxy-calls a claim_bid on the Auction contract, telling it to dump the winner's payment into an escrow account called accept_payment on the AuctionManager struct. It has the same token type as the auction. Once all payments have been collected, the front end then calls the empty_payment_account endpoint one time (since you are the only creator on the Metadata being sold) and the funds in this escrow are paid out to a token account provided of the same type owned by you.

Note that our front end reference implementation uses SOL as the "token type." This has some special caveats, namely that SOL isn't really an "spl token." It instead has a work-around called the "Wrapped SOL mint." This is a special mint that is often used in a transient account. What this means is that when we place a bid, we actually make a one-off system account, transfer lamports to it of your bid amount + rent, then label it an spl-token account of the wrapped sol type, use it to place the bid, then close it all in one transaction.

Special machinations in the spl token program then make this wrapped sol token account have a number of tokens with the proper decimals that map to the amount of SOL you transferred to it. We do a similar operation with cancelling, where we make a transient wrapped sol account, transfer cancelled bid funds to it, and then close the account, transferring funds out all in a single transaction. This is all done for ease of use. With settlements, when funds are disbursed to artists, we actually make a WSOL account for them and they have to close it by hand via a dropdown menu.

The protocol operates off of generic spl tokens and has no opinions about WSOL specifically, but the front end reference implementation does. So take careful note!
Original doc source: metaplex



The important links:
https://github.com/HashLips/hashlips_...
https://github.com/metaplex-foundatio...
https://github.com/exiled-apes/candy-...

Set the environment:
solana config set --url https://api.devnet.solana.com

The important commands:
npx ts-node js/packages/cli/src/candy-machine-cli.ts upload ./assets --env devnet --keypair (PATH)

npx ts-node js/packages/cli/src/candy-machine-cli.ts create_candy_machine --env devnet --keypair (PATH) 

npx ts-node js/packages/cli/src/candy-machine-cli.ts update_candy_machine --keypair (PATH) --price 1 --date "19 Oct 2021 00:00:00 EST"

Note: for the main net you want to replace all instances of "devnet" with "mainnet-beta".

Also when you are done just run "yarn build" to build the app that you can host on a website.
source: https://www.youtube.com/watch?time_continue=169&v=35RO0lAEIxE&feature=emb_logo

https://solscan.io/


# Compete in tardigrade battles (2+ players - bet on winner + winner collects claws)
## 

# Exchange tardigrades (metaplex?)

# Minimalist game GUI (Unity? Heatmap rendering?)
https://github.com/GabeDottl/unity-solana-wallet

# Abstract into more generic API
(i.e. drop the tardigrades)
https://heroiclabs.com/docs/nakama/client-libraries/unity-client-guide/#nakama-client


# Useful repos
Metaplex NFT platform: https://github.com/metaplex-foundation/metaplex


# Useful Guides
Tons of Cloudflare Workers docs
https://developers.cloudflare.com/workers/examples

Looks like Figment is doing a pretty thorough 'how-to-use Solana' course:
https://learn.figment.io/protocols/solana

Sweet guide explaining Solana's technical benefits with less technical jargon:
https://learn.figment.io/tutorials/explaining-solanas-innovations-without-technical-jargon

Really great Solana developer documentation store:
https://www.soldev.app/

Build a pay-to-play gaming platform on Solana (42 pages printed!?)
https://learn.figment.io/tutorials/pay-to-play-gaming-on-solana

Unity has great tutorials for starting game developers:
https://learn.unity.com/tutorial/working-with-animations-and-animation-curves?signup=true#600c8a28edbc2a3270428113

This repository is the data source for the Solana Ecosystem page, located at solana.com/ecosystem:
https://github.com/SuperChad69/add-superchad-ecosystem

The game engine you waited for. Godot provides a huge set of common tools, so you can just focus on making your game without reinventing the wheel.

Godot is completely free and open-source under the very permissive MIT license. No strings attached, no royalties, nothing. Your game is yours, down to the last line of engine code.
https://godotengine.org/
^Active interest in Solana discords.

Godot won't limit your creative endeavours, as it's capable of handling almost every project you can think of.

"Godot is an incredibly versatile engine whose limits in terms of occlusion culling and 3D performance are likely not more than a few months from being negligible," says Ryan Hewer, project director at Little Red Dog Games, referring to the release of Godot 4.0 later this year.

"At present, there's very, very little where I would say 'Godot is not best for that', with the exception of possibly very detailed first and third-person shooters. But with Godot 3.2 -- and more importantly Godot 4.0 -- you're going to see parity with most any other mainstream engine out there. Godot is advanced enough that I'm comfortable saying that most developers will be bottlenecked by their own skills rather than the tools in front of them."
source: https://www.gamesindustry.biz/articles/2020-04-14-what-is-the-best-game-engine-is-godot-right-for-you

# Miscellaneous current news
https://solflare.com/academy/solanas-nft-ecosystem/#:~:text=How%20do%20Solana%20NFTs%20work%3F%20Solana%20NFTs%20are,govern%20how%20tokens%20operate%20on%20the%20Solana%20blockchain.
https://github.com/zubr-exchange/cacherpc - Solana JSON-RPC caching server.
https://docs.solana.com/cluster/rpc-endpoints

# Questions
https://lwus.github.io/metaplex/ - what is this?
