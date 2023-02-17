main :: IO ()
main =
  putStrLn "Try to guess the secret to quit the program!" >> func ""

func :: String -> IO ()
func "foo" = putStrLn "Bye Bye"
func "" = putStr "secret> " >> getLine >>= func
func _ = putStr "Try again!\nsecret> " >> getLine >>= func
