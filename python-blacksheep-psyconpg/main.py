import os
import asyncpg
from dataclasses import dataclass
from dotenv import load_dotenv
from blacksheep import Application


app = Application()
load_dotenv()


@app.on_start
async def before_start(application: Application) -> None:
    app.pool = await asyncpg.create_pool(os.getenv("DATABASE_URL"), min_size=0, max_size=5)


@app.route("/")
async def index():
    words = []

    async with app.pool.acquire() as conn:
        async with conn.transaction():
            rows = await conn.fetch("SELECT id, title, content from words limit 100")

    for row in rows:
        word = {"id": row[0], "title": row[1], "content": row[2]}
        words.append(word)

    return words


@app.route("/ping")
def ping():
    return "OK"
