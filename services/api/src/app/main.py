from fastapi import FastAPI

from .routers import v1

app = FastAPI(title="MLDL Bench API")

app.include_router(v1.router, prefix="/v1")

@app.get("/healthz")
def healthz():
    return {"status": "ok"}


