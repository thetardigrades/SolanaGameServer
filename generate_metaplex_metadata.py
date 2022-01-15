from glob import glob
import os.path
import json

def generate(target_path):
  target_path = os.path.abspath(target_path)
  for path in glob(os.path.join(target_path, "*png"),recursive=True):
    subpath = path[len(target_path):]
    base, _ = os.path.splitext(subpath)
    with open(f'{base}.json', 'w+') as metadata_file:
      metadata_file.write()
    
