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

Ha - skipped this: https://docs.metaplex.com/candy-machine-v2/configuration (see config.json)
https://docs.civic.com/candy-machine-integration/adding-captcha-to-candy-machine-v2#terms-and-conditions - huh, civic.

# TODO: for SGS, https://spl.solana.com/token#example-wrapping-sol-in-a-token
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
Excellent CMv2 walkthrough (1/10/22): https://twitter.com/marcelc63/status/1480570145193951234
https://github.com/kevinfaveri/solana-candy-factory
https://github.com/exiled-apes/candy-machine-mint
Bot to post to Discord when NFTs sell: https://github.com/milktoastlab/SolanaNFTBot


# Breed minted tardigrades

# Compete in tardigrade battles (2+ players - bet on winner + winner collects claws)

# Exchange tardigrades (metaplex?)

# Minimalist game GUI (Unity? Heatmap rendering?)
https://github.com/GabeDottl/unity-solana-wallet

# Abstract into more generic API
(i.e. drop the tardigrades)
https://heroiclabs.com/docs/nakama/client-libraries/unity-client-guide/#nakama-client


# Useful repos
Metaplex NFT platform: https://github.com/metaplex-foundation/metaplex


# Useful Guides
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

# Miscellaneous current news
https://solflare.com/academy/solanas-nft-ecosystem/#:~:text=How%20do%20Solana%20NFTs%20work%3F%20Solana%20NFTs%20are,govern%20how%20tokens%20operate%20on%20the%20Solana%20blockchain.

# Questions
https://lwus.github.io/metaplex/ - what is this?