Solana Game Server* is the first decentralized Game Server designed for game developers & web3.

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
Not there.

## API locked?
Nope.

## Are there tests?
Where there are APIs, there will be tests.

## How can I help?
Reach out if you're a game developer interested in using our platform.

## Where's the Discord?
Avoiding the noise - checkout tardigrades.online for contact info, or make an issue here.

## Where's the latest info?
See Notes.md.

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

## Can I fork/self-host and sell parts of this?
No, nor should you. Aside from legalities, it's not an economically useful thing to do. It takes
time and money to build & maintain any form of infrastructure. With any software infrastructure
(including B2B-friendly smart contracts, such as this), it takes time and effort to support forks.
The software development time-cost and trust-cost associated with using an unofficial fork exceeds
the relative benefit of forking - particularly since patches are welcome.

Self-hosting is also expensive - it can cost thousands of dollars to run QA against the new release
and.

Companies who are interested in direct-support (e.g. integrating your game's mechanics) - make your
pitch in the public issues or feature requests. If it's of interest to anyone (either me or anyone
I can pay to implement the feature), then it will be built & integrated on a B2B contract basis.

Only trusted code will be supported, and will be behind appropriate versioning, testing, and CI/CD infra.

Private or limited-release feature launches are also supported for an additional fee (consulting).

## Where's the pitch deck / whitepaper?
https://docs.google.com/document/d/15X4FWa8i-tR7nsjn_CncPbc80zsGmiMD9bMKOUmTzzI/edit#

## Why Solana?
"From all the “Ethereum killers”, Solana emerged in 2021 as the hottest one, with the most engaged developer community. Through a unique blend of eight technological innovations (including Proof of History), it alleviates many of the problems that plague other blockchains and has been able to reach record levels of speed and transaction volumes." - source: https://learn.figment.io/protocols/solana

## Why gaming middleware?
Game development has it's own unique requirements for backend systems to facilitate game mechanics (breeding, battle systems, characters, items, etc.)

## Whats with the Tardigrades?
They're cute - see https://www.tardigrades.online.
