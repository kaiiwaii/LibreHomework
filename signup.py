
import requests
"""
url = "https://librehomework-api.herokuapp.com/signup"

data={"username": "hgepro", "password":"Hectorg2007","email":"hgepro@protonmail.com", "bio":"Creator of LibreHomework", "discord": "0xHGE#4808"}

print(requests.post(url, data=data).text)
"""
"""
#login
url = "https://librehomework-api.herokuapp.com/login"
data = {"username": "hgepro", "password":"Hectorg2007"}
tk = requests.post(url, data=data).json()["auth-token"]


url = "https://librehomework-api.herokuapp.com/edit"
data = {"token": tk, "bio": "testing", "email": "test"}
print(requests.post(url, data=data).text)
"""
for i in range(1, 1000):
    url = "https://librehomework-api.herokuapp.com/dailymessage"
    print(requests.get(url).text)