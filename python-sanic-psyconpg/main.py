import os
import asyncpg
from dataclasses import dataclass
from dotenv import load_dotenv
from sanic import Sanic
from sanic.response import text, json


app = Sanic("MyApp")
load_dotenv()

@app.listener("before_server_start")
async def before_server_start(app, loop):
    app.ctx.pool = await asyncpg.create_pool(os.getenv("DATABASE_URL"), min_size=0, max_size=5)


@app.get("/")
async def index(request):
    words = []

    async with app.ctx.pool.acquire() as conn:
        rows = await conn.fetch("SELECT id, title, content from words limit 100")

    for row in rows:
        word = {"id": row[0], "title": row[1], "content": row[2]}
        words.append(word)

    return json(words)


@app.get("/ping")
def ping(request):
    return text("OK")
