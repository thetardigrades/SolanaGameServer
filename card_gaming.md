# Goal
Building card games with NFTs using Metaplex feels like a probable common use-case
- and a useful starting point. Minimize metaplex functionality-duplication, maximize use.

# Possible Needs
* Define a deck of cards with counts and other universe*-specific attributes

*: Seems like defining groupings of games which are inter-compatible as 'universes'
makes sense. Need some 'group of compatible cards' name.

# Possible Entities
* Player: A collection of characters, player-attributes, and historical information
* Character: A NFT asset
* Item: A different type of NFT asset
* A NFT asset / 'Card': A Tardigrades-tagged NFT for easy identification on-chain
* A Universe: A collection of NFT assets which are game-level compatible.
* A Species: A collection of NFT assets which are species-level breading capable.
* An Alliance: A collection of Species which are aligned / 'allies'.

# Game
Tardigrades MTG-style. 
A Tardigrade card/instance/collectible/NFT can be defined as:
(version.visual_style.source_pattern.nft_cost.artist_universe.claw_count)

Tardigrades may compete in claw counting competitions where the number of claws dictates
the better Tardigrade.

Tardigrades wear with age - lose claws after 'competitions'. Tardigrades can be bread
to get deterministic results (baby tardigrades can be auctioned). Tardigrades have
potential value which varies with age according to a fixed schedule + actions.

Ideally, rendered in-browser using Phantom and minimal wallet-interaction.

Global games are fun with minimal chance, mostly timing - as long as you can keep
cheating low (see Heart Token).

## Functionality
There are 4 key functions:
1) Mint Tardigrades (using Candy Machine(?))
2) Breed minted tardigrades ()
3) Compete in tardigrade battles (2+ players - bet on winner + winner collects claws)
4) Exchange tardigrades (metaplex?)

It'd be fun if claws are made default exchange currency for tardigrade NFTs. They a
NFT which carries a value whose.

Target bottom of the market with tardigrade art/NFTs which can grow in strength over
time - they have 'intrinsic value' in the game/metaverse. Should be pay-to-earn
ideally where the more active * skilled you are, the more you are able to create with
the tardigrades. If this is successful, then anti-cheating mechanisms will be
integrated (Tardigrada Colony,Village = Beta launch worlds; Tardigrada City = Alhpa launch world).

Fun app development lifcycle (ALC) ideas:
* Claws are currency and have value/weight as a function cver their lineage.
* Text-based interface? text dungeon-crawler stlye
* Solana does Pokemon

Fun software development lifecycle (SDLC) ideas:
* Editor is terminal or schema-based rather than web UI - better ergo & easier
* Tokenomics modeling & projection as a service

Ideal features:
* Minimum data & contained within Solana's tiny compute limits
* Maximum number of concurrent, global players
* Minimum waiting for players (instant gratification)
* Long-term time-based effects
* Cross-game card reuse/compatibility
* Functional backend+storage purely on solana+arweave (minus client assets)

