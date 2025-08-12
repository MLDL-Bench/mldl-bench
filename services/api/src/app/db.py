from pathlib import Path
from typing import AsyncIterator
import aiosqlite

DB_PATH = Path(__file__).resolve().parents[2] / "data" / "mldl.db"


async def get_db() -> AsyncIterator[aiosqlite.Connection]:
    DB_PATH.parent.mkdir(parents=True, exist_ok=True)
    async with aiosqlite.connect(DB_PATH) as db:
        db.row_factory = aiosqlite.Row
        yield db


