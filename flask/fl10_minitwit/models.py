from flask_sqlalchemy import SQLAlchemy
from sqlalchemy.orm import DeclarativeBase, mapped_column, relationship
from sqlalchemy import Integer, String, Text, Column, ForeignKey


class Base(DeclarativeBase):
    pass


db = SQLAlchemy(model_class=Base)


class User(db.Model):
    user_id = mapped_column(Integer, primary_key=True)
    username = mapped_column(String(24), nullable=False)
    email = mapped_column(String(80), nullable=False)
    pw_hash = mapped_column(String(64), nullable=False)

    messages = relationship("Message", back_populates="author")

    follows = relationship(
        "User",
        secondary="follows_table",
        primaryjoin="User.user_id==follows_table.c.follower_id",
        secondaryjoin="User.user_id==follows_table.c.followee_id",
        back_populates="followed_by",
    )
    followed_by = relationship(
        "User",
        secondary="follows_table",
        secondaryjoin="User.user_id==follows_table.c.follower_id",
        primaryjoin="User.user_id==follows_table.c.followee_id",
        back_populates="follows",
    )

    def __init__(self, username, email, pw_hash):
        self.username = username
        self.email = email
        self.pw_hash = pw_hash

    def __repr__(self):
        return "<User {}>".format(self.username)


follows_table = db.Table(
    "follows_table",
    Column("follower_id", Integer, ForeignKey("user.user_id")),
    Column("followee_id", Integer, ForeignKey("user.user_id")),
)


class Message(db.Model):
    message_id = mapped_column(Integer, primary_key=True)
    author_id = mapped_column(Integer, ForeignKey("user.user_id"), nullable=False)
    text = mapped_column(Text, nullable=False)
    pub_date = mapped_column(Integer)

    author = relationship("User", back_populates="messages")

    def __init__(self, author_id, text, pub_date):
        self.author_id = author_id
        self.text = text
        self.pub_date = pub_date

    def __repr__(self):
        return "<Message {}".format(self.message_id)
