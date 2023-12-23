import requests
from bs4 import BeautifulSoup

url = input("input xiaoyuzhoufm link:")

response = requests.get(url)
soup = BeautifulSoup(response.content, "html.parser")

title_tag = soup.find("meta", {"property": "og:title"})
audio_tag = soup.find("meta", {"property": "og:audio"})

title = title_tag["content"]
audio_url = audio_tag["content"]

response = requests.get(audio_url)

with open(f"{title}.m4a", "wb") as f:
    f.write(response.content)

print(f"audio {title}.m4a downloaded!")
