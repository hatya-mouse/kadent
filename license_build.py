import json
import sys
from types import NoneType
from typing import TypedDict

output_path = (sys.argv[1:2] or ['license.json'])[0]
crate_licenses = (sys.argv[2:3] or ['crate_licenses.json'])[0]
font_licenses = (sys.argv[3:4] or ['assets/fonts/font_licenses.json'])[0]
notices_path = (sys.argv[4:5] or ['notices.json'])[0]
priority_list_path = (sys.argv[5:6] or ['license_priority.json'])[0]

class LicenseItem(TypedDict):
    name: str
    id: str
    text: str
class DependencyItem(TypedDict):
    name: str
    version: str | NoneType
    authors: list[str]
    description: str | NoneType
    license_id: str
    license_index: int
    notice: str | NoneType

notices: dict[str, str] = {}
license_priority: list[str] = []
licenses: list[LicenseItem] = []
fonts: list[DependencyItem] = []
crates_dict: dict[tuple[str, str], DependencyItem] = {}

def compare_licenses(a_id: str, b_id: str) -> bool:
    return license_priority.index(a_id) < license_priority.index(b_id)

def add_license(license: LicenseItem) -> int:
    licenses.append(license)
    return len(licenses) - 1

# Load notices list
with open(notices_path, 'r') as notices_file:
    notices = json.load(notices_file)['notices']

# Load license priority list
with open(priority_list_path, 'r') as priority_file:
    license_priority = json.load(priority_file)['priority']

# --- CRATES (Result of cargo-about) ---
with open(crate_licenses, 'r') as crate_file:
    crate_json = json.load(crate_file)

    for license in crate_json['licenses']:
        license_item: LicenseItem = {
            "name": license['name'],
            "id": license['id'],
            "text": license['text']
        }
        license_index = add_license(license_item)

        for crate in license['used_by']:
            crate = crate['crate']
            notice = notices.get(crate['name'], None)
            dependency: DependencyItem = {
                "name": crate['name'],
                "version": crate['version'],
                "authors": crate['authors'],
                "description": crate['description'],
                "license_id": license['id'],
                "license_index": license_index,
                "notice": notice
            }

            crate_key = (crate['name'], crate['version'])
            existing_crate = crates_dict.get(crate_key)
            if existing_crate is not None:
                existing_license_id = str(existing_crate['license_id'])
                if compare_licenses(license['id'], existing_license_id):
                    crates_dict[crate_key] = dependency
            else:
                crates_dict[crate_key] = dependency

# --- FONTS ---
with open(font_licenses, 'r') as font_file:
    font_json = json.load(font_file)

    for font in font_json['fonts']:
        license = font['license']
        with open(license['path'], 'r', encoding='utf-8') as license_file:
            license_text = license_file.read()
            license_index = add_license({
                "name": license['name'],
                "id": license['id'],
                "text": license_text
            })

            notice = notices.get(font['name'], None)
            dependency: DependencyItem = {
                "name": font['name'],
                "version": None,
                "authors": font['authors'] if not font['authors'] is NoneType else [],
                "description": None,
                "license_id": license['id'],
                "license_index": license_index,
                "notice": notice
            }
            fonts.append(dependency)

crates = list(crates_dict.values())

# Remove unused licenses
used_indices: set[int] = {crate['license_index'] for crate in crates} | {font['license_index'] for font in fonts}
final_license_indices: list[int] = []
final_licenses: list[LicenseItem] = []

for index, license in enumerate(licenses):
    if index in used_indices:
        final_license_indices.append(len(final_licenses))
        final_licenses.append(license)
    else:
        final_license_indices.append(-1)

for crate in crates:
    crate['license_index'] = final_license_indices[crate['license_index']]

for font in fonts:
    font['license_index'] = final_license_indices[font['license_index']]

# Dump the data to a JSON file
json_str = json.dumps(
    {
        "licenses": final_licenses,
        "crates": crates,
        "fonts": fonts
    },
    ensure_ascii=False,
    indent=4
)

with open(output_path, "w", encoding="utf-8") as file:
    _ = file.write(json_str)
