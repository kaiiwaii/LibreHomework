from cryptography.fernet import Fernet, InvalidToken
import datetime
import os, struct
import traceback

#from env import ENV
fer = Fernet(os.environ["TOKEN_KEY"].encode("utf8"))
#fer = Fernet(ENV["TOKEN_KEY"].encode("utf8"))


def generate_token(username,ttl):
  global fer
  data = b""
  data += os.urandom(16)
  data += struct.pack(">I",ttl)
  data += username.encode("utf-8")
  return fer.encrypt(data)

def validate_token(token):
    global fer
    try:
        created_at = datetime.datetime.utcfromtimestamp(fer.extract_timestamp(token))

        data = fer.decrypt(token)[16:]
        ttl = int.from_bytes(data[:4], byteorder="big")
        username = data[4:].decode("utf8")

        diff = datetime.datetime.utcnow() - created_at

        if diff.total_seconds() / 60 > ttl:
            return False, None
            print("Expired token")
        else:
            return True, username

    except InvalidToken:
        return False, None