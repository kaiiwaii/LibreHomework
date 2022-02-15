from hashlib import blake2s, md5

def hash(password):
    return blake2s(password).hexdigest()

def get_gravatar(email):
    if email:
        return "https://www.gravatar.com/avatar/" + md5(email.encode("utf8")).hexdigest()
    else:
        return None