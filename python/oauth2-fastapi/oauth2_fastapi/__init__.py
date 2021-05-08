# based on tutorial: https://www.youtube.com/watch?v=6hTRw_HK3Ts&t=90s
from pathlib import Path

import jwt
from fastapi import Depends, FastAPI, HTTPException
from fastapi.security import OAuth2PasswordBearer, OAuth2PasswordRequestForm
from passlib.hash import bcrypt
from tortoise import fields
from tortoise.contrib.fastapi import register_tortoise
from tortoise.contrib.pydantic import pydantic_model_creator
from tortoise.models import Model

app = FastAPI()
oauth2_scheme = OAuth2PasswordBearer(tokenUrl="token")

BASE_PATH = Path(__file__).parent
JWT_SECRET = (BASE_PATH / "JWT_SECRET").read_text()


class User(Model):
    id = fields.IntField(pk=True)
    username = fields.CharField(50, unique=True)
    password_hash = fields.CharField(128)


UserPydantic = pydantic_model_creator(User, name="User")
UserInPydantic = pydantic_model_creator(User, name="UserIn", exclude_readonly=True)


@app.post("/token")
async def generate_token(form_data: OAuth2PasswordRequestForm = Depends()):
    user = await User.get(username=form_data.username)
    if not bcrypt.verify(form_data.password, user.password_hash):
        raise HTTPException(401, "invalid credentials")

    user_obj = await UserPydantic.from_tortoise_orm(user)
    token = jwt.encode(user_obj.dict(), JWT_SECRET)
    return {"access_token": token, "token_type": "bearer"}


@app.get("/")
async def index(token: str = Depends(oauth2_scheme)):
    return {"the_token": token}


@app.post("/users", response_model=UserPydantic)
async def create_user(user: UserInPydantic):
    user_obj = User(
        username=user.username, password_hash=bcrypt.hash(user.password_hash)
    )
    await user_obj.save()
    return await UserPydantic.from_tortoise_orm(user_obj)


@app.get("/users/me", response_model=UserPydantic)
async def get_user(token: str = Depends(oauth2_scheme)) -> UserPydantic:
    payload = jwt.decode(token, JWT_SECRET, algorithms=["HS256"])
    user = await User.get(id=payload.get("id"))
    return await UserPydantic.from_tortoise_orm(user)


register_tortoise(
    app,
    db_url="sqlite://db.sqlite3",
    modules={"models": ["oauth2_fastapi"]},
    generate_schemas=True,
    add_exception_handlers=True,
)
