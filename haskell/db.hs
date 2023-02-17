{-# LANGUAGE OverloadedStrings #-}

import Database.SQLite.Simple (close, execute_, open, query_)
import Database.SQLite.Simple.FromRow

data TestField = TestField Int String deriving (Show)

instance FromRow TestField where
  fromRow = TestField <$> field <*> field

main :: IO ()
main =
  open "test.db"
    >>= \conn ->
      execute_
        conn
        "CREATE TABLE IF NOT EXISTS test (id INTEGER PRIMARY KEY, str TEXT)"
        >> (query_ conn "SELECT * from test" :: IO [TestField])
        >>= print
        >> close conn
