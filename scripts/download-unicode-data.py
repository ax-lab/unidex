#! /usr/bin/env python3

import io
import os
import zipfile

from urllib import request

UNICODE_SITE = 'https://www.unicode.org'
UCD_ZIP_FILE = 'Public/UCD/latest/ucd/UCD.zip'

OUTPUT_DIRECTORY_UCD = 'data/ucd'


root_dir = os.path.join(os.path.dirname(__file__), '..')


def process_ucd():
    print()
    data_dir = os.path.join(root_dir, OUTPUT_DIRECTORY_UCD)
    if os.path.isfile(os.path.join(data_dir, 'Index.txt')):
        print(
            '=> UCD data seems to exist already, skipping...\n   (to download again delete `%s` contents)' %
            (OUTPUT_DIRECTORY_UCD))
        return

    os.makedirs(data_dir, exist_ok=True)
    ucd_url = '%s/%s' % (UNICODE_SITE, UCD_ZIP_FILE)
    print('=> Downloading %s...' % ucd_url)
    with request.urlopen(ucd_url) as response:
        if response.status != 200:
            print('   ...download failed with status %d' % response.status)
            return
        data = response.read()
        print('=> Downloaded %d bytes, extracting...' % len(data))
        with zipfile.ZipFile(io.BytesIO(data)) as zip:
            zip.extractall(data_dir)
            print('   ...UCD files extracted to `%s`' % OUTPUT_DIRECTORY_UCD)


def main():
    process_ucd()


if __name__ == "__main__":
    main()
