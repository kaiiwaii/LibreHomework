from hashlib import blake2s, md5
import random, string

def salt(length):
    letters = string.ascii_letters
    return ''.join(
        random.choice(letters) for _ in range(length)
    ).encode()

def hash(password, s=None):
    if s and type(s) == bytes:
        pass
    else:
        s = salt(10)
    return (blake2s(password + s).hexdigest(), s)

def get_gravatar(email):
    if email:
        return "https://www.gravatar.com/avatar/" + md5(email.encode("utf8")).hexdigest()
    else:
        return None