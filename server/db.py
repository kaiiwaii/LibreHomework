import aiosqlite
import utils
import authtoken

async def setup_tables(db):
        async with db.cursor() as c:
            await c.execute("""
        CREATE TABLE IF NOT EXISTS users (
            username VARCHAR(32) PRIMARY KEY,
            password VARCHAR(64) NOT NULL,
            email VARCHAR(320), 
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            discord VARCHAR(32),
            twitter VARCHAR(16),
            bio VARCHAR(50));
            """) #note that email is public
            

async def list_users(db, page):

    temp = []
    async with db.cursor() as c:
        await c.execute("SELECT username, email FROM users LIMIT 20 OFFSET ?", (page * 20,))
        for row in await c.fetchall():
            temp.append({row[0]: utils.get_gravatar(row[1])})
        
    return temp

    
async def add_user(db, username, password, email, discord, twitter, bio):
    password = utils.hash(password.encode("utf8"))
    async with db.cursor() as c:
        try:
            q = await c.execute("""
            INSERT INTO users (username, password, email, discord, twitter, bio)
            VALUES (?, ?, ?, ?, ?, ?)
            """, (username, password, email, discord, twitter, bio,))
            await db.commit()

            if q.rowcount == 0:
                return False
            else:
                return True
        except Exception as e:
            print(e)
            return False


async def remove_user(db, token):
    res, username = authtoken.validate_token(token)
    if res:
        async with db.cursor() as c:
            q = await c.execute("DELETE FROM users WHERE username = ?", (username,))
            await db.commit()
            if q.rowcount == 0:
                return False
            else:
                return True
    else:
        return False


async def login(db, username, password):
    async with db.cursor() as c:
        await c.execute("""
        SELECT username FROM users where password = ?
        """, (utils.hash(password.encode("utf8")),))

        db_username = await c.fetchone()
        if db_username[0] == username:
            return True, authtoken.generate_token(username, 15) #15 minutes
        else:
            return False



async def find_user(db, username):
    temp = []
    async with db.cursor() as c:
        await c.execute("""
    SELECT username, email, created_at, discord, twitter, bio FROM users WHERE username LIKE ? LIMIT 10
    """, (username,))
        for row in await c.fetchall():
            temp.append({"username": row[0], "email": row[1], "creation_date": row[2],"discord": row[3], "twitter": row[4], "bio": row[5], "profile_picture": utils.get_gravatar(row[1])})
    
    return temp


async def edit_user(db, token, email, discord, twitter, bio):
    res, username = authtoken.validate_token(token)
    if res:
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
        query += " WHERE username = ?"
        args.append(username)

        async with db.cursor() as c:
            q = await c.execute(query, args)
            await db.commit() 
            if q.rowcount == 0:
                return False
            else:
                return True
    else:
        return False
