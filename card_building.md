Building card games with NFTs using Metaplex feels like a probable common use-case
- and a useful starting point. Minimize metaplex functionality-duplication, maximize use.

Possible Needs:
* Define a deck of cards with counts and other universe*-specific attributes

*: Seems like defining groupings of games which are inter-compatible as 'universes'
makes sense. Need some 'group of compatible cards' name.

Possible Entities:
* Player: A collection of characters, player-attributes, and historical information
* Character: A NFT asset
* Item: A different type of NFT asset
* A NFT asset / 'Card': A Tardigrades-tagged NFT for easy identification on-chain
* A Universe: A collection of NFT assets which are game-level compatible.
* A Species: A collection of NFT assets which are species-level breading capable.
* An Alliance: A collection of Species which are aligned / 'allies'.

Game:
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

There are 3 key functions:
1) Mint Card using Candy Machine(?)
2) Breed 
2) 'Battle' cards [delayed-trigger or instant?]

Ideal features:
* Minimum data & contained within Solana's tiny compute limits
* Maximum number of concurrent, global players
* Minimum waiting for players (instant gratification)
* Long-term time-based effects
* Cross-game card reuse/compatibility
* Functional backend+storage purely on solana+arweave (minus client assets)


Fun ideas:
* Editor is terminal or schema-based rather than web UI - better ergo & easier
