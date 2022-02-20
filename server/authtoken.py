from cryptography.fernet import Fernet
import datetime
import os, struct

fer = Fernet(os.environ["TOKEN_KEY"].encode("utf8"))

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
            return False
        else:
            return True, username

    except Exception as e:
        print(traceback.format_exc())
        return False