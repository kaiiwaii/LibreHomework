from hashlib import blake2s, md5
import random, string
import os, asyncio, aiofiles


salt = os.environ["SALT"].encode("utf8")

def hash(password):
    return blake2s(password + salt).hexdigest()

def get_gravatar(email):
    if email:
        return "https://www.gravatar.com/avatar/" + md5(email.encode("utf8")).hexdigest()
    else:
        return None

async def update_daily_message(app):
    while True:
        async with aiofiles.open("dailymessage.txt", "r") as f:
            app.ctx.dailymessage = await f.read()
        await asyncio.sleep(600)