import os
import asyncpg
from dataclasses import dataclass
from fastapi import FastAPI
from dotenv import load_dotenv


app = FastAPI()
load_dotenv()


@app.on_event("startup")
async def startup():
    app.state.pool = await asyncpg.create_pool(os.getenv("DATABASE_URL"), min_size=0, max_size=5)


@app.get("/")
async def index():
    words = []

    async with app.state.pool.acquire() as conn:
        rows = await conn.fetch("SELECT id, title, content from words limit 100")

    for row in rows:
        word = {"id": row[0], "title": row[1], "content": row[2]}
        words.append(word)

    return words


@app.get("/ping")
def ping():
    return "OK"
