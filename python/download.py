"""Script for downloading corpora files from https://data.riksdagen.se/Data/Dokument/."""

import html
import re
import urllib.request
import urllib.parse
from pathlib import Path

URL = "https://data.riksdagen.se/Data/Dokument/"
RAWDIR = "rawdata"

def download():
    r = urllib.request.urlopen(URL)
    html_page = r.read().decode("utf8")

    urls = re.findall(r'"//data\.riksdagen\.se/dataset/dokument/\S+\.xml\.zip"', html_page)
    if not urls:
        raise("No URLs found according to pattern on https://data.riksdagen.se/Data/Dokument/")

    urls = ["https:" + html.unescape(u).strip('"') for u in urls]

    for url in urls:
        name = url.split('/')[-1]
        filepath = Path(RAWDIR) / name
        encoded_url = "/".join(url.split('/')[:-1]) + "/" + urllib.parse.quote(name)
        if filepath.is_file():
            print(f"Skipping {filepath}")
            continue
        print(f"Downloading {filepath} ({encoded_url})...")
        try:
            urllib.request.urlretrieve(encoded_url, filepath)
        except urllib.request.HTTPError:
            print(f"Error: failed to download {encoded_url}")


if __name__ == "__main__":
    download()
