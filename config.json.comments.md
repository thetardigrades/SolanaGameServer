# Specifications
https://docs.metaplex.com/candy-machine-v2/configuration

# "expireOnUse": true
^I feel like this should be false for a better game experience, but erring towards safety.

# Wallet
rmJDUHfhfyVSBkPrX6AYDYLG5yh9oWk2Sb7k1tHCQwN: TLS, Inc's wallet.

# Token (SGS)
Minted (devnet) 'SGS' token 5cSmecmjfcQULy5RhRWvFsTsG5TNRdqC4esn7HEewuqs - 10,000 supply.
Used for toll mechanism to use public smart contracts.

# Token Account (SGS)
SGSACCOUNT=7ahL3HAW76NCK7tvNXwmyCR1Nzvwjxz37GAaefa6r4EF

# "storage": "aws"
“aws”	Uploads to AWS (must specify AWS Bucket name)
Tried other options - too buggy.

# Gatekeeper
"While the unpredictable mint index provides some protection against bots, bots are still able to mint directly from the Candy Machine. If you want to make sure that only humans can mint from your project, you can enable the gatekeeper settings in your config.json with the following values:"
"gatekeeper": {
    "gatekeeperNetwork" : "ignREusXmGrscGNUesoU9mxfds9AiYTezUKex2PsZV6",
    "expireOnUse" : true
}


CAUTION
The number of items setting can only be updated after you create your CMv2 if you are using hiddenSettings. When hiddenSettings are not used, the number value is used to allocate the space required by the CMv2 account and therefore cannot be modified.

In case you require to change the number of items after creating a CMv2 without hiddenSettings, you can withdraw rent of your current CMv2 and then create a new one.

# Unused
"ipfsInfuraProjectId": null,
"ipfsInfuraSecret": null,
"awsS3Bucket": null,
"noRetainAuthority": false,
"endSettings": null,
"whitelistMintSettings": null,
"hiddenSettings": null

