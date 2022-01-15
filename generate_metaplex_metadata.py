'''Copyright 2022 by Tardigrade Life Sciences, Inc (tardigrades.online)'''

from glob import glob
# TODO: Use orjson for performance boost?
import json
import os
import os.path
import shutil


def generate(target_path):
  print(f'{__file__}: generate')
  target_path = os.path.abspath(target_path)
  print(f'target_path: {target_path}')
  for path in glob(os.path.join(target_path, "*png"), recursive=True):
    print(path)
    basepath, _ = os.path.splitext(path)
    metadata_filename = f'{basepath}.json'
    subpath = path[len(target_path) + 1:]
    image_name, _ = os.path.splitext(subpath)

    continent, city, country, artist, index = image_name.split('.')
    print(f'continent, city, country, artist, index: {(continent, city, country, artist, index)} ')
    with open(metadata_filename, 'w+') as metadata_file:
      # https://docs.metaplex.com/candy-machine-v2/preparing-assets
      d = {
          # Spec: https://docs.metaplex.com/token-metadata/v1.1.0/specification#token-standards
          'name': image_name,
          "description": f"T{index}: A tardigrade by {artist} from {city}, {country}, {continent}.",
          "seller_fee_basis_points": 500,
          "image":
              "https://tardigrades.art/assets/XXX",  #"https://arweave.net/26YdhY_eAzv26YdhY1uu9uiA3nmDZYwP8MwZAultcE?ext=jpeg",
          # "animation_url": "https://arweave.net/ZAultcE_eAzv26YdhY1uu9uiA3nmDZYwP8MwuiA3nm?ext=glb",
          # TODO: Cloudflare mirror? Or some decentralized equiv?
          "external_url": "https://tardigrades.art/assets/XXX",
          # TODO:
          "properties": {
              "files": [
                  {
                      "uri": f"https://tardigrades.art/assets/{index}.ext",  #www.arweave.net/abcd5678?ext=png",
                      "type": "image/png"
                  },
                  {
                      "uri": f"https://tardigrades.art/assets/{index}.ext",  #watch.videodelivery.net/9876jkl",
                      "type": "unknown",
                      "cdn": True
                  },
                  {
                      "uri": f"https://tardigrades.art/assets/{index}.ext",  #www.arweave.net/efgh1234?ext=mp4",
                      "type": "video/mp4"
                  }
              ],
              "creators": [{
                  # TODO: parameterize/inject with some tooling.
                  "address": "rmJDUHfhfyVSBkPrX6AYDYLG5yh9oWk2Sb7k1tHCQwN",
                  "share": 100
              }],
              "category": "video",
          },
          "collection": {
              "name": "numbers",
              "family": "numbers"
          },
      }

      # TODO: get these on-chain & compressed, and standardized.
      d["attributes"] = [{'trait_type': 'origin', 'value': f'{city}, {country}, {continent}'}]
      metadata_file.write(f"{json.dumps(d, indent=2)}\n")
    print(metadata_filename)
  print('finished')


def build(target_path):
  print(f'{__file__}: build')
  target_path = os.path.abspath(target_path)
  print(f'target_path: {target_path}')
  build_dir = os.path.join(target_path, 'build')
  if os.path.exists(build_dir):
    shutil.rmtree(build_dir)
  os.mkdir(build_dir)
  print(f'Created {build_dir}')
  for i, (img, metadata) in enumerate(
      zip(glob(os.path.join(target_path, '*png')), glob(os.path.join(target_path, '*json')))):
    print(f"src={img}, dst=f'{target_path}/{i}'")
    print(f"src={metadata}, dst=f'{target_path}/{i}'")
    shutil.copyfile(src=img, dst=f'{target_path}/{i}.png')
    shutil.copyfile(src=metadata, dst=f'{target_path}/{i}.json')


if __name__ == '__main__':
  target_path = os.path.join(os.path.dirname(__file__), 'sample_data')
  generate(target_path)
  build(target_path)
