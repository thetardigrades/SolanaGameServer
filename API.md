Think decentralized AWS Games: Solana (compute), arweave (storage), HeartToken (auth & fraud prevention) and Cloudflare (security, CDN).
Or Steam for crypto freemium games

Game Server APIs:
* Announce (queryable, public, tagged, ASCII)
* Get, List, Create, Delete (optimizing for tiny representation & minimal compute)
* Public Group Chat (via temporary memos to transaction history)
* Native discord integration (bots)
* Announcements (via temporary memos to transaction history)
* Marketplace (via Serum)

Note: I think the least-complex starting path for significant storage would be transient memos stored on arweave via transaction history. 
These can't be used in-contract, but could be read-in by the client for storing historical information within a game.
Contract APIs:
* Configure Game rules
* Execute game-step
* Configure player stats
* Affect environment
* Variant setting (for turninging on/off features via DAO)

Game developer notes:
Nakama, Photon, and NodeJS are commonly used for multiplayer

Massive Online Battle Arenas could be interesting on Solana.
* Up to N players with M bits of data and simple operations.
* Tournaments

Follow graph - sign memo to 'follow' other users (off chain)

https://heroiclabs.com/

Solana Candy Factory
https://github.com/kevinfaveri/solana-candy-factory

Integrate unity-solana-wallet

Database-like APIs? (via SQLite3 or query engine over data?)
https://stackoverflow.com/questions/2011724/whats-the-best-database-for-embedded

Redis-like API? High-efficiency object storage and compute.

Paid APIs:
* Extract-Transform-Load APIs for doing analytics on your games

Questions:
What're the most common on-chain actions for games?
