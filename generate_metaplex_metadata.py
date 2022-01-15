'''Copyright 2022 by Tardigrade Life Sciences, Inc (tardigrades.online)'''

from glob import glob
import json
import os.path


def generate(target_path):
  print(f'{__file__}: generate')
  target_path = os.path.abspath(target_path)
  print(f'target_path: {target_path}')
  for path in glob(os.path.join(target_path, "*png"), recursive=True):
    print(path)
    basepath, _ = os.path.splitext(path)
    metadata_filename = f'{basepath}.json'
    subpath = path[len(target_path)+1:]
    image_name, _ = os.path.splitext(subpath)
    
    continent, city, country, artist, index = image_name.split('.')
    print(f'continent, city, country, artist, index: {(continent, city, country, artist, index)} ')
    with open(metadata_filename, 'w+') as metadata_file:
      # https://docs.metaplex.com/candy-machine-v2/preparing-assets
      d = {
          # Spec: https://docs.metaplex.com/token-metadata/v1.1.0/specification#token-standards
          'name': image_name,
          "description": f"T{index}: A tardigrade by {artist} from {city}, {country}, {continent}.",
          "image": "https://tardigrades.art/XXX", #"https://arweave.net/26YdhY_eAzv26YdhY1uu9uiA3nmDZYwP8MwZAultcE?ext=jpeg",
          # "animation_url": "https://arweave.net/ZAultcE_eAzv26YdhY1uu9uiA3nmDZYwP8MwuiA3nm?ext=glb",
          # TODO: Cloudflare mirror? Or some decentralized equiv?
          "external_url": "https://tardigrades.art/XXX",
          # TODO: 
          "properties": {
              "files": [{
                  "uri": f"https://tardigrades.art/{index}.ext", #www.arweave.net/abcd5678?ext=png",
                  "type": "image/png"
              }, {
                  "uri": f"https://tardigrades.art/{index}.ext", #watch.videodelivery.net/9876jkl",
                  "type": "unknown",
                  "cdn": True
              }, {
                  "uri": f"https://tardigrades.art/{index}.ext", #www.arweave.net/efgh1234?ext=mp4",
                  "type": "video/mp4"
              }],
              "category": "video"
          }
      }

      # TODO: get these on-chain & compressed, and standardized.
      d["attributes"] = [{'trait_type': 'origin', 'value': f'{city}, {country}, {continent}'}]
      metadata_file.write(json.dumps(d))
    print(metadata_filename)
  print('finished')


if __name__ == '__main__':
  generate(os.path.join(os.path.dirname(__file__), 'sample_data'))

# Source: https://docs.metaplex.com/token-metadata/v1.1.0/specification#token-standards
# {
#   "name": "SolanaGame Steel Sword",
#   "symbol": "SG-SS-1",
#   "description": "SolanaGame steel sword available after Level 4",
#   "image": "<https://arweave.net/26YdhY_eAzv26YdhY1uu9uiA3nmDZYwP8MwZAultcE?ext=jpeg>",
#   "animation_url": "<https://arweave.net/ZAultcE_eAzv26YdhY1uu9uiA3nmDZYwP8MwuiA3nm?ext=glb>",
#   "external_url": "<https://SolanaGame.io>",
#   "attributes": [
#     {
#       "trait_type": "attack",
#       "value": "4"
#     },
#     {
#       "trait_type": "defense",
#       "value": "3"
#     },
#     {
#       "trait_type": "durability",
#       "value": "47"
#     },
#     {
#       "trait_type": "components",
#       "value": "iron: 10; carbon: 1; wood: 2"
#     }
#   ]
# }

# {
#   "name": "SolanaArtProject #1",
#   "description": "Generative art on Solana.",
#   "image": "https://arweave.net/26YdhY_eAzv26YdhY1uu9uiA3nmDZYwP8MwZAultcE?ext=jpeg",
#   "animation_url": "https://arweave.net/ZAultcE_eAzv26YdhY1uu9uiA3nmDZYwP8MwuiA3nm?ext=glb",
#   "external_url": "https://example.com",
#   "attributes": [
#     {
#       "trait_type": "trait1",
#       "value": "value1"
#     },
#     {
#       "trait_type": "trait2",
#       "value": "value2"
#     }
#   ],
#   //@deprecated -> do not use - may be removed in a future release
#   "collection": {
#      "name": "Solflare X NFT",
#      "family": "Solflare"
#   },
#   "properties": {
#     "files": [
#       {
#         "uri": "https://www.arweave.net/abcd5678?ext=png",
#         "type": "image/png"
#       },
#       {
#         "uri": "https://watch.videodelivery.net/9876jkl",
#         "type": "unknown",
#         "cdn": true
#       },
#       {
#         "uri": "https://www.arweave.net/efgh1234?ext=mp4",
#         "type": "video/mp4"
#       }
#     ],
#     "category": "video",
#     //@deprecated -> do not use - may be removed in a future release
#     "creators": [
#       {
#         "address": "xEtQ9Fpv62qdc1GYfpNReMasVTe9YW5bHJwfVKqo72u",
#         "share": 100
#       }
#     ]
#   }
# }