import argparse

def main():
    parser = argparse.ArgumentParser(description="Download and extract a ZIP file from a URL into a specified directory.")
    parser.add_argument(
        "url",
        help="The URL of the ZIP file to download."
    )
    parser.add_argument(
        "dir",
        help="The directory to extract the contents into."
    )

    args = parser.parse_args()
    url = args.url
    dir = args.dir

    import os
    import zipfile
    import requests
    from io import BytesIO
    response = requests.get(url)
    with zipfile.ZipFile(BytesIO(response.content)) as z:
        z.extractall(dir)

    # adds dir/zipfile/rust/ where zipfile is the name of the zip file without extension
    zipfile_name = os.path.splitext(os.path.basename(url))[0]
    os.makedirs(os.path.join(dir, zipfile_name, "rust"), exist_ok=True)

    print(f"Downloaded and extracted to {dir}")

if __name__ == "__main__":
    main()