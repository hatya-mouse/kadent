import json
import sys

args = sys.argv
crate_licenses = sys.argv[0] or 'crate_licenses.json'
font_licenses = sys.argv[1] or 'font_licenses.json'

type LicenseItem = dict[str, str]
type DependencyItem = dict[str, str | list[str] | int]

licenses: list[LicenseItem] = []
fonts: list[DependencyItem] = []
crates: list[DependencyItem] = []

current_license_id = 0

def gen_license_id():
    global current_license_id
    current_license_id += 1
    return current_license_id

def create_license(license_name: str, license_id: str, license_text: str):
    return {
        "name": license_name,
        "id": license_id,
        "text": license_text
    }

def create_dependency(name: str, version: str, authors: list[str], description: str, license_id: int):
    return {
        "name": name,
        "version": version,
        "authors": authors,
        "description": description,
        "license_id": license_id,
    }

# --- CRATES (Result of cargo-about) ---
with open(crate_licenses, 'r') as crate_file:
    crate_json = json.load(crate_file)

    for crate_license in crate_json['licenses']:
        license_id = gen_license_id()
        license = create_license(crate_license['name'], crate_license['id'], crate_license['text'])
        licenses[license_id] = license

        for crate in crate_license['used_by']:
            dependency = create_dependency(crate['name'], crate['version'], crate['authors'], crate['description'], license_id)
            crates.append(dependency)

# --- FONTS ---
with open(font_licenses, 'r') as font_file:
    font_json = json.load(font_file)
    licenses.extend(font_json['licenses'])
    fonts = font_json['fonts']

# Dump the data to a JSON file
json_str = json.dumps(
    {
        "licenses": licenses,
        "crates": crates,
        "fonts": fonts
    },
    ensure_ascii=False,
    indent=4
)

with open("sample.txt", "w", encoding="utf-8") as file:
    _ = file.write(json_str)
