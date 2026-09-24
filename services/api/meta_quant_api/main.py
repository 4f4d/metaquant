# SPDX-License-Identifier: Apache-2.0
from fastapi import FastAPI

app = FastAPI(title="META QUANT API", version="0.1.0")


@app.get("/health")
def health() -> dict[str, str]:
    return {"status": "ok", "mode": "paper"}


@app.get("/version")
def version() -> dict[str, str]:
    return {"version": "0.1.0"}
