from fastapi import APIRouter
from pydantic import BaseModel
from typing import Any, List, Optional
from uuid import UUID, uuid4
from datetime import datetime, timezone

router = APIRouter()


class DeviceSpec(BaseModel):
    id: Optional[UUID] = None
    hostname: str
    os: str
    cpu_model: Optional[str] = None
    cpu_cores: Optional[int] = None
    memory_gb: Optional[float] = None
    gpu_model: Optional[str] = None
    gpu_driver: Optional[str] = None


class BenchmarkDescriptor(BaseModel):
    name: str
    version: str


class RunRequest(BaseModel):
    device: DeviceSpec
    benchmark: BenchmarkDescriptor
    parameters: Any


class RunResult(BaseModel):
    run_id: UUID
    started_at: datetime
    completed_at: datetime
    metrics: Any
    artifacts: List[str]


class SignedResult(BaseModel):
    result: RunResult
    signature: str


@router.get("/ping")
async def ping():
    return {"pong": True}


@router.post("/runs", response_model=SignedResult)
async def submit_run(request: RunRequest) -> SignedResult:
    now = datetime.now(timezone.utc)
    result = RunResult(
        run_id=uuid4(),
        started_at=now,
        completed_at=now,
        metrics={"accepted": True},
        artifacts=[],
    )
    signed = SignedResult(result=result, signature="insecure-dev-signature")
    return signed


