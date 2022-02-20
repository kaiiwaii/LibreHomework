from hashlib import blake2s, md5
import random, string
import os

salt = os.environ["SALT"].encode("utf8")

def hash(password):
    return blake2s(password + salt).hexdigest()

def get_gravatar(email):
    if email:
        return "https://www.gravatar.com/avatar/" + md5(email.encode("utf8")).hexdigest()
    else:
        return None