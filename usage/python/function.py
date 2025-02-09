from redis import Redis

redis = Redis(protocol=3)


def handler(event, context):
    hits = redis.get("hits") or "0"
    hits = int(hits) + 1
    redis.set("hits", hits)
    return {"hits": hits}
