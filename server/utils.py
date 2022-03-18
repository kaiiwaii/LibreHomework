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

async def get_daily_message(app):
    while True:
        #reformat with db usage
        async with app.ctx.db.acquire() as conn:
            result = await conn.fetch("SELECT * FROM daily_messages ORDER BY id DESC LIMIT 1")
            try:
                app.ctx.dailymessage = result[0]["message"]
            except Exception as e:
                app.ctx.daily_message = "Nothing to see here"
                print(e)
        await asyncio.sleep(600)