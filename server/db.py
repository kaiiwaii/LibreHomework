import aiosqlite
import utils

async def setup_tables(db):
        async with db.cursor() as c:
            await c.execute("""
        CREATE TABLE IF NOT EXISTS users (
            username VARCHAR(32) PRIMARY KEY,
            password VARCHAR(64) NOT NULL,
            salt VARCHAR(10) NOT NULL,
            email VARCHAR(320), 
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            discord VARCHAR(32),
            twitter VARCHAR(16),
            bio VARCHAR(50));
            """)
            

async def list_users(db, page):

    temp = []
    async with db.cursor() as c:
        await c.execute("SELECT username FROM users LIMIT 20 OFFSET ?", (page * 20,))
        for row in await c.fetchall():
            temp.append(row[0])
        
    return temp

    
async def add_user(db, username, password, email, discord, twitter, bio):
    hp = utils.hash(password.encode("utf8")) # Get hashed password and the used salt
    async with db.cursor() as c:
        await c.execute("""
        INSERT INTO users (username, password, salt, email, discord, twitter, bio)
        VALUES (?, ?, ?, ?, ?, ?, ?)
        """, (username, hp[0], hp[1], email, discord, twitter, bio,))
        await db.commit()

        if q.rowcount == 0:
            return False
        else:
            return True


async def remove_user(db, username, password):
    async with db.cursor() as c:
        q = await c.execute("""
            DELETE FROM users WHERE username = ? AND password = ?
            """, (username, utils.hash(password.encode("utf8")),))
        await db.commit()

        if q.rowcount == 0:
            return False
        else:
            return True



async def login():
    pass


async def update_user(db, username, password, key, value):
    return 


async def find_user(db, username):
    temp = []
    async with db.cursor() as c:
        await c.execute("""
    SELECT username, email, discord, twitter, bio FROM users WHERE username LIKE ? LIMIT 10
    """, (username,))
        for row in await c.fetchall():
            temp.append({"username": row[0], "email": row[1], "discord": row[2], "twitter": row[3], "bio": row[4], "profile_picture": utils.get_gravatar(row[1])})
    
    return temp


async def edit_user(db, username, password, email, discord, twitter, bio):
    query = "UPDATE users SET "
    args = []

    if email:
        query += "email = ?, "
        args.append(email)
    if discord:
        query += "discord = ?, "
        args.append(discord)
    if twitter:
        query += "twitter = ?, "
        args.append(twitter)
    if bio:
        query += "bio = ?, "
        args.append(bio)

    query = query[:-2]
    query += " WHERE username = ? AND password = ?"
    args.append(username)
    args.append(utils.hash(password.encode("utf8")))

    async with db.cursor() as c:
        q = await c.execute("""
    UPDATE users SET email = ?, discord = ?, twitter = ?, bio = ? WHERE username = ? AND password = ?
    """, args)
        await db.commit() 
        if q.rowcount == 0:
            return False
        else:
            return True