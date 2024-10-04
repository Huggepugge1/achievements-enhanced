import requests
import sys
import os

print("Checking for updates")

# Repository details
owner = "Huggepugge1"
repo = "achievements-enhanced"

# GitHub API endpoint for releases
release_url = f"https://api.github.com/repos/{owner}/{repo}/releases/latest"

# Send GET request for the latest release
response = requests.get(release_url)
tag_name = response.json()["tag_name"]

# Check if the latest release is newer than the current version
if tag_name != sys.argv[1]:
    print(f"Update available: {sys.argv[1]} -> {tag_name}")
    print("Do you want to update? (y/n)")
    choice = input().lower()
    if choice == "y":
        # Download the latest release
        download_url = response.json()
        for asset in download_url["assets"]:
            if os.name == "nt":
                if asset["name"] == "achievements_enhanced.exe":
                    download_url = asset["browser_download_url"]
                    break
            else:
                if asset["name"] == "achievements_enhanced":
                    download_url = asset["browser_download_url"]
                    break
        else:
            print("Error: No download found")
            exit()

        download_response = requests.get(download_url)
        download = requests.get(download_url)
        print("Downloaded latest release")

        if os.name == "nt":
            with open("achievements_enhanced.exe", "wb") as file:
                file.write(download.content)

        else:
            with open("achievements_enhanced", "wb") as file:
                file.write(download.content)
            os.system("chmod +x achievements_enhanced")
        
        print("Update installed")
        print("Restart the program to apply changes")
