import uvicorn


def start():
    """Launched with `poetry run start` at root level"""
    uvicorn.run(
        "oauth2_fastapi:app",
        host="0.0.0.0",
        port=8000,
        reload=True,
        reload_dirs=["oauth2_fastapi"],
    )


start()