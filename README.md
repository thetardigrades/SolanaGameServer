Solana Game Server* is the first decentralized Game Server designed for game developers & web3.

(Think web3 SDK for game developers as a service)

Web2 gaming today is entrenched by large game studios and aging freemium model franchises.

Web2 game models suck and web3 has the potential to unlock novel differentiation and monetization
mechanisms for small and large games alike.

The most popular 'crypto games' today are limited in their complexity by their blockchain (Ethereum)
or the complexity of building secure, interesting blockchain-based mechanics (Solana et al).

Game developers are held back from crypto by a few holdups:
1) Poor performance and scaling on EVM-compatible blockchains
2) Poor infrastructure support on most performant chain (Solana) compared to EVM infrastructure
   and terrible in comparison to web2 gaming infrastructure.
3) Complexity of the constantly evolving language of gamefi/defi/nft/crypto/blockchain space is
   dizzying to most people - game developers need a translation layer.
4) Crypto is still broadly confusing, sketchy and a big target for hackers - which makes it
   harder for newer blockchain developers to be successful with web2 strategies.

SGS endeavors to make it easier for game developers to take their web2 games and adapt them to
web3 with Solana by building the best, secure, game-oriented smart contracts on Solana.

Why? Because Game development on Solana is *challenging*. Reference: See this *42p* document on
how to build play-to-earn Solana games: https://learn.figment.io/tutorials/pay-to-play-gaming-on-solana.

It's possible to build amazing games on Solana but it's a PITA and TLS wants to make it as easy
as building a web2 game.

Code will be published here and deployed to Solana mainnet periodically. Fees (TBD) will be baked
into the smart contract. A CDN + Endpoint solution will be provided as a service using said fees.
The contract will allow creating custom coins and operating within the Solana Game Server. Code
& documentation is public by-default to encourage community involvement.

This will all be exposed either on-chain directly or via a SaaS generating & deploying contracts on-demand.

Possible games: card battles, turn-based X, breeding, land ownership, metaverse avatar customization

For best understanding of the current state of this project, see todo_and_status.md.

*Solana Game Server is not directly owned or maintained by Solana Labs. The name 'Solana Game Server'
is used instead of a codename to reduce ambiguity. Name suggestions welcome.

permalink: solanagameserver.com
web3 games (WIP): tardigrades.fun

possible new names: web3 game maker, <suggestions>

solana stake-authorize $W3 --stake-authority $W1 --new-stake-authority  \
    --fee-payer <KEYPAIR>

# Possible features (sorted by priority)
- Echo + Log and Broadcast Messages
- Stable, free non-blockchain API for testing
- User and character stats
- Data storage cost estimation
- World management
- Asset (fungible or non-fungible) ownership
- Wallet management and integration
- Land ownership
- Farming mechanics
- On-chain battle execution systems
- Governance systems
- Virtual pet mechanics
- Breeding mechanics
- Auction mechanics
- Standardized Tokenomics structures
- Gambling

# Issues / Feature Requests
Are you a game developer looking for a feature or solution for your game's backend? Add a GitHub issue or reach out directly.

# Help wanted
Are you an experienced Rust programmer looking for Solana exposure or a Game developer? Reach out.

## Opportunities
* Create a fancy GitHub documentation site
* Design & Build a game ontop of the demo network
* Design APIs for game developers

# License
SGS is owned and operated by Tardigrade Life Sciences, Inc. Contact hi@tardigrades.store for licensing/use options.

# FAQ
## How do I request features?
Make an Issue or a pull request. Issues with offered eng-assistance are prioritized. If you want specific features, you can
'vote' for them by mkaing an issue (lower ranking) or adding them directly to feature_requests.md.

## Contributor agreement
Contributions are welcome. TLS maintains perpetual license to use any code submitted to the codebase in perpetuity under the
existing license.

If your legal department requires more info or formalization, contact me.

## What's the mainnet address?
Not launched yet.

## What's the testnet address?
Still not launched yet.

## What's the devnet address?
Still not launched yet.

## Where's the IDL?!?
Not there yet.

## Where's the API? Is it locked?
See API.md for latest work or todo_and_status.md. The API is not yet locked / requests are welcome.

## Are there tests?
Where there are APIs, there will be tests.

## How can I help?
Reach out if you're a game developer interested in using our platform.

## Where's the Discord?
Avoiding the noise - checkout tardigrades.online for contact info, or make an issue here.

## Where's the latest info?
See Notes.md.

## What's the business model?
Tentatively: Selling/providing Game passes (crypto payment or CC) which fund Solana-based transactions in your game's metaverse
for your web2 users. You can have cool NFT-generating mechanics and collectibles combined with
a dirt-cheap, global-scale ~2FPS, 2MB gameserver / global state machine.

## Why is there a LICENSE?
Short-answer: To balance a need for openness with a need for sustainable development funding.

Openness: Building OSS for more esoteric platforms (like Solana) is different than building it for more
popular general use-cases (e.g. databases). There will probably be relatively few
early users of this platform (although if it's successful, we'll of course expand) and of those,
most will fail but a few will hopefully thrive. We want to provide a needed solution for game
developers and studios looking to invest in building global-scale crypto games on the fastest
L1 blockchain. Building software on top of software is *much* easier when the software you're
building on is public, modifiable, and maintained for you.

Many deployed smart contracts are pay-for-use, yet private source - which is undesireable for a 'trustless' network. I respect
wanting to maintain competitive edge by not sharing information with competitors, but I think at this stage in the ecosystem,
it's preferable to be trusted as the 'official' source over other projects which may (illegally) fork and commercialize without
licensing.

Funding: Writing good software is hard and I'd like Tardigrade Life Sciences, Inc (TLS or Tardigrades) to be able to get funding from users.
Also, businesses (e.g. game studios, institutions) like working with other businesses with formal, contractual relationships. By
giving leaving ownership & management of these contracts to Tardigrade Life Sciences, my expectation/hope is to save many lost engineering
hours by game developers trying to build on Solana.

Also, as per the License, this software is property of TLS and thus increases TLS's intrinsic value,
regardless of whether or not the software is directly monetized (e.g. for acquisition).

Please note, TLS is not presently associated with any venture funding as of January 2022.

## Can I fork/self-host and sell parts of this anyway?
Technically yes, but should you, probably not. Aside from legalities, it's not an economically
useful thing to do. It takes time and money to build & maintain any form of infrastructure.
With any software infrastructure (including B2B-friendly smart contracts, such as this), it takes
time and effort to support personal forks - even direct copy-pastes.

The software development time, security and trust-costs associated with using an unofficial fork
exceeds the relative benefit of forking - particularly since patches (security and beta features)
are welcome.

Self-hosting is also expensive - it can cost thousands of dollars to run QA against the new release
and hundreds of dollars to deploy new versions of code to mainnet.

It's also kinda a job itself to self-host (system reliability engineering / infrastructure engineering).
The contributors(s) of this codebase should enjoy building scalable infrastructure.

Companies who are interested in direct-support (e.g. integrating your game's mechanics) - make your
pitch in the public issues or feature requests. If it's of interest to anyone (either me or anyone
I can pay to implement the feature), then it will be built & integrated on a B2B contract basis.

Only trusted code will be supported, and will be behind appropriate versioning, testing, and CI/CD infra.

Private or limited-release feature launches are also supported for an additional fee (consulting).

## How to use in my library/unity app/etc.?
See API & feature__requests.md.

## Recommended tools and alternatives?
(recommend your alternative tool/projects here via a pull request)
Metaplex - https://hackmd.io/@levicook/HJcDneEWF

Also, https://github.com/paul-schaaf/awesome-solana.

## Where's the pitch deck / whitepaper?
https://docs.google.com/document/d/15X4FWa8i-tR7nsjn_CncPbc80zsGmiMD9bMKOUmTzzI/edit#

## Why Solana?
"From all the “Ethereum killers”, Solana emerged in 2021 as the hottest one, with the most engaged developer community. Through a unique blend of eight technological innovations (including Proof of History), it alleviates many of the problems that plague other blockchains and has been able to reach record levels of speed and transaction volumes." - source: https://learn.figment.io/protocols/solana

Visa-scale + permanent, public transaction history + trustless compute = best web3 game experience*

*: Assuming the infrastructure is in place
https://solana.com/news/proof-of-history---a-clock-for-blockchain

In effect, you have faster, cheaper, more reliable compute (but waaaay less volatile memory)
than other blockchains.

Most other inexpensive blockchain optimize more towards maximizing memory - which makes them good for
NFTs, but bad for complex games.

Unfortunately, Ethereums ease-of-use is partially due to its permissiveness which is substantially greater
than Solana (somewhat analogous to programming in Python on ethereum with fixes happening at the VM-level
vs. low-level C embedded-systems-style programming on Solana). Anchor helps a lot with developer ergonomics, but doesn't
change the intrinsic complexity of low-level distributed systems development.

## Is there really a market for crypto gaming?
Maybe? Looking at how many web2 freemium games monetize (buying in-app, digital goods) and 
* The internet seems to agree: https://trends.google.com/trends/explore?geo=US&q=gamefi,%2Fg%2F11fpjwyy5z,%2Fg%2F11f345g8p9,NBA%20TopShot
* VC capital seems to agree with its funding (see recent rounds).
* Economics of NFTs, supply of game developer talent, plus perpetual, global hunger for new content

Blockchain interest over time: https://trends.google.com/trends/explore?date=today%205-y&q=blockchain,ethereum,solana,polygon,matic%20network

## Why gaming middleware?
Game development has it's own unique requirements for backend systems to facilitate game mechanics (breeding, battle systems, characters, items, etc.)

## Subscribe for early access?
Contact me via tardigrades.online.

## Is Solana performant enough for my game idea?
As the software infrastructure and best practices evolve, SGS should allow game developers to
have a high-speed, low-bandwidth global game engine @ Solana performance levels. See
Solana labs demo:
https://github.com/solana-labs/break

Note that the demo is using a 'test' network - actual mainnet performance will take months-to-years
to match. Good starting games should rely on game mechanics that can take advantage of ~1mb of shared
global state easily and that cost-for-compute (including deserialization) is high.

Looking at gameplay mechanics from existing crypto games is a good starting point for understanding
how to take advantage of a 'programmable trustless platform' (i.e. a smart contract blockchain, like Ethereum or Flow).

## Whats with the Tardigrades?
They're cute - see https://www.tardigrades.online.

# Like this? Interested in GameFi (blockchain gaming)? web3 gaming? 'trustless VR metaverses'?
Share this with your favorite game developers or ask them what platform-level features they'd like built.

Don't forget to like & subscribe. tardigrades.online
