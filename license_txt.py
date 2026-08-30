import json
import sys
from types import NoneType
from typing import TypedDict

licenses_path = (sys.argv[1:2] or ["licenses.json"])[0]
output_path = (sys.argv[2:3]or ["licenses.txt"])[0]

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

licenses: list[LicenseItem] = []
fonts: list[DependencyItem] = []
crates: list[DependencyItem] = []

# Load Licenses File
with open(licenses_path, 'r') as licenses_file:
    licenses_json = json.load(licenses_file)

    for license in licenses_json["licenses"]:
        licenses.append(license)

    for crate in licenses_json["crates"]:
        crates.append(crate)

    for font in licenses_json["fonts"]:
        fonts.append(font)

with open(output_path, 'w') as f:
    f.write("ライセンス情報\n\n");
    f.write("本作品では、以下のOSSおよびフォントを使用しています。\n各ライセンスの全文および著作権表示は、Kadentアプリケーション内のAcknowledgementsから確認できます。\n\n");
    f.write("フォント\n\n");

    for font in fonts:
        _ = f.write(f"- {font["name"]}\n");
        if font["version"]:
            _ = f.write(f"  バージョン: {font["version"]}\n");
        if font["authors"]:
            _ = f.write(f"  作者: {", ".join(font["authors"])}\n");
        _ = f.write(f"  ライセンス: {licenses[font["license_index"]]["name"]}\n\n");

    f.write("クレート\n\n");

    for crate in crates:
        _ = f.write(f"- {crate["name"]}\n");
        if crate["version"]:
            _ = f.write(f"  バージョン: {crate["version"]}\n");
        if crate["authors"]:
            _ = f.write(f"  作者: {", ".join(crate["authors"])}\n");
        _ = f.write(f"  ライセンス: {licenses[crate["license_index"]]["name"]}\n\n");
