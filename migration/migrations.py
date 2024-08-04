import json

class ConfigFile:
    def __init__(self, path, content):
        self.path = path
        self.content = content

    def load(path):
        try:
            with open(path, encoding='utf-8') as f:
                return ConfigFile(path, json.load(f))
        except Exception as e:
            print(e)
            print(f'Failed to read {path}')
            return None

    def dump(self):
        with open(self.path, 'w', encoding='utf-8') as f:
            json.dump(self.content, f, indent=4)


def update_version(root_file, repos, old_version, new_version):
    files = [root_file] + repos
    for config_file in files:
        if config_file.content['version'] != old_version:
            return False
        config_file.content['version'] = new_version
    return True

# For updates that only require to modify the version field
def auto_bump_version(old_version, new_version):
    def bump(root_file, repos):
        if not update_version(root_file, repos, old_version, new_version):
            return False
        return True

    return bump

def migrate_0_13_to_0_14(root_file, repos):
    if not update_version(root_file, repos, '0.13', '0.14'):
        return False

    # Add an empty cc field to the repos
    for r in repos:
        for s in r.content['series']:
            s['cc'] = ''

    return True