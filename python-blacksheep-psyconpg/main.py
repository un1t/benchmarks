import os
from dataclasses import dataclass
from dotenv import load_dotenv
from blacksheep import Application
from psycopg_pool import AsyncConnectionPool

app = Application()
load_dotenv()
db_pool = AsyncConnectionPool(os.getenv("DATABASE_URL"), min_size=0, max_size=3)


@app.route("/")
async def index():
    words = []

    async with db_pool.connection() as conn:
        async with conn.cursor() as cur:
            await cur.execute("SELECT id, title, content from words limit 100")
            rows = await cur.fetchall()

    for row in rows:
        word = {"id": row[0], "title": row[1], "content": row[2]}
        words.append(word)

    return words


@app.route("/ping")
def ping():
    return "OK"
