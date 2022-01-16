Note: If you're curious to see existing SOTA game ergonomics on Solana: https://learn.figment.io/tutorials/pay-to-play-gaming-on-solana

https://www.cnet.com/tech/computing/the-metaverse-isnt-what-you-think-it-is-because-we-dont-know-what-it-is/
Public bet: The move into 'passports' or 'gaming' is FB building a self-sovereign identity plaform (like @H)

config.json is CMv2: https://docs.metaplex.com/candy-machine-v2/configuration

# TODO
 * Review Unity Multiplay API as an API starting point.
 * MVP use-case:
 * Propose Game Developer-oriented APIs for proposed features
 * https://heroiclabs.com/docs/nakama/client-libraries/unity-client-guide/ & https://www.photonengine.com/en/PUN
 * Check vulnerabilities: https://github.com/project-serum/sealevel-attacks/tree/master/programs
 * Much Later: Anti-cheating mechanics
 * https://github.com/metaplex-foundation/metaplex-program-library/blob/master/.github/workflows/program-candy-machine.yml

Core features:
Wallet web integration plugin (solana unity plugin?)
Create in-game currency
Create in-game character NFT (using NFT art generator)
Transfer account NFT (some royalty mechanisms)
Decentralized asset serving

Choose your order of operations - bid on execution
GameSolPool? executes (free) as long as Pool has money. Pool used to run game executions.
Invite stick + group ban (off-chain vote)

More interesting on-chain features:
Affect character gameplay state
Affect environment gameplay state
Triggered-effects
Define game mechanics
Define transition rules
Run rule over environment for each character

On-chain reputation for Solana apps
* Reputation systems 
https://fatboardgames.com/5-most-common-game-mechanic/
* Worker placement, engine building, Card Draft, Tile Placement, Cooperative

* On-chain moderation
battle resolution system: bfs attacks

Breed character NFT
3D generative in-game NFTs?

CI/CD for Solana Devnet?

Straightforward add-ons:
Wallet custodianship
Online/Offline (and other) bit flips
Create New Coins
Transfer In/Out coins

External References:
* https://defiland.app/ - DeFi Land is a multi-chain agriculture-simulation game created to gamify Decentralized Finance. Our game will have all the features that traditional platforms have but it will be gathered all in one place. Players will be able to start playing the game for free, grind their way up to the play-2-earn tier, compete with others, craft, trade and more.
* https://www.solarity.xyz/ - "Nobody should be able to rule the Metaverse so it’s vital to develop a common
  infrastructure that enables anyone to develop and deploy their dreams for a better future." - similar idea,
  different approach (and **very** sketch - they're Twitter was taken down...).
* https://www.soldev.app/
* Candy Machine v2 (Multiplex NFT minter): https://twitter.com/marcelc63/status/1480570145193951234
* Excellent Anchor IDL Tweet thread: https://twitter.com/jozanza/status/1481098724008992772
* https://solarians.click/gallery - excellent example of beautiful, playful, yet incredibly simple Solana game.
  * https://solarians.click/roadmap - roadmap is kinda fluffy becuase decentralized game state is hard to do.
* https://projectseed.io/ - game-oriented roadmap; good first user?
* https://github.com/bmresearch/Solnet - "Solana's .NET SDK and integration library."

# Miscellaneous news
"Integrating blockchain in games well is so hard is because most of the infrastructure that’s actually needed doesn’t exist, isn’t mature enough, or isn’t great for games" - https://venturebeat.com/2021/05/20/forte-raises-185m-at-1b-valuation-for-blockchain-game-platform/
https://github.com/Polygon-Academy/Web3GamingBootcamp_CN - Polygon + web3 gaming - ChainIDE: "A cloud based blockchaining API" - for EVMs

# Cool new repos
https://github.com/trending/rust?since=monthly
https://github.com/bevyengine/bevy - A refreshingly simple data-driven game engine built in Rust
https://github.com/tauri-apps/tauri - Build smaller, faster, and more secure desktop applications with a web frontend.
https://github.com/dani-garcia/vaultwarden - BitWarden for Rust
https://github.com/alacritty/alacritty - A cross-platform, OpenGL terminal emulator.
https://github.com/RustScan/RustScan - brew install rustscan
https://github.com/trending/developers/rust?since=monthly - find top Rust devs
https://github.com/gakonst/foundry - Foundry is a blazing fast, portable and modular toolkit for Ethereum application development written in Rust.