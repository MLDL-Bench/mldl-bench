Run locally:

```bash
cd services/api
uv venv && uv pip install -e .
uvicorn app.main:app --reload
```

Run migrations:

```bash
cd services/api
alembic upgrade head
```


