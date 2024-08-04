#! /usr/bin/env python3

import os
import sys
import sys
import json
import argparse
from shutil import make_archive
from datetime import datetime

from migrations import *


# Must be ordered
VERSIONS = [
    ((0, 14), migrate_0_13_to_0_14),
    ((0, 15), auto_bump_version('0.14', '0.15')),
]


def detect_version(root_file):
    version = root_file.content.get('version', None)
    if version is None:
        return None
    t = version.split('.')
    for i, elt in enumerate(t):
        t[i] = int(elt)
    return tuple(t)


def get_subsequent_versions(version):
    for i, v in enumerate(VERSIONS):
        if v[0] > version:
            return VERSIONS[i:]
    return []


def parse_args():
    parser = argparse.ArgumentParser()
    parser.add_argument("root_dir")
    return parser.parse_args()


def get_root_file_abspath(root_dir):
    return os.path.abspath(os.path.join(root_dir, 'root.json'))


def load_files(root_dir):
    root_file_path = get_root_file_abspath(root_dir)
    
    root_file = ConfigFile.load(root_file_path)
    if root_file is None:
        return None
    
    repos = []
    for dir_elt in os.listdir(root_dir):
        dir_elt_full_path = os.path.abspath('/'.join((root_dir, dir_elt)))

        if dir_elt_full_path == root_file_path:
            continue

        if os.path.isfile(dir_elt_full_path):
            c = ConfigFile.load(dir_elt_full_path)
            if c is None:
                return None
            repos.append(c)

    return root_file, repos


def create_backup(root_dir):
    backup_name = f'migration_backup_{datetime.now().strftime("%Y_%m_%d_%H_%M_%S")}'
    return make_archive(backup_name, 'zip', root_dir)


def replace_files(root_file, repos):
    try:
        root_file.dump()
        for r in repos:
            r.dump()
    except Exception as e:
        print(e)
        print('Migration failed, consider restoring from the backup')
        sys.exit(1)


def main():
    args = parse_args()
    root_dir = args.root_dir

    load_res = load_files(root_dir)
    if load_res is None:
        sys.exit(1)
    root_file, repos = load_res

    version = detect_version(root_file)
    if version is None:
        print('Failed to detect current version')
        sys.exit(1)

    migrations = get_subsequent_versions(version)
    if len(migrations) == 0:
        print('Already up to date')
        sys.exit(0)

    for m in migrations:
        print(f'migration to {".".join(map(str, m[0]))}')
        if not m[1](root_file, repos):
            sys.exit(1)

    # If the migration is complete, we can write the files back
    backup = create_backup(root_dir)
    print(f'Backup created : {backup}')
    replace_files(root_file, repos)
    print('Files updated')

if __name__ == '__main__':
    main()
