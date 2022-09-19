import Data.List
import Data.List.Split

test1 = "-ab+ba"

-- "ab+ba-2cd" -> "ab", "+ba", "-2cd"

tokenize :: [String] -> String -> [String]
tokenize acc [] = fmap reverse acc
tokenize acc ('+' : rest) = "+" : acc
tokenize acc ('-' : rest) = "-" : acc
tokenize (start : restAcc) (next : rest) = (next : start) : restAcc

main :: IO ()
main = print $ sort <$> splitOn "+" test1

signStringToInt :: String -> Int
signStringToInt "-" = -1
signStringToInt _ = 1

amountStringToInt :: String -> Int
amountStringToInt "" = 1
amountStringToInt x = read x

monomials :: String -> [Monomial]
monomials "" = []
monomials str = Monomial cnt variables : monomials rest
  where
    (signStr, afterSign) = span (\x -> x == '-' || x == '+') str
    (amountStr, afterAmount) = span isDigit afterSign
    (variablesStr, rest) = span isLetter afterAmount
    cnt = signStringToInt signStr * amountStringToInt amountStr
    variables = sort variablesStr
    signStringToInt "-" = -1
    signStringToInt _ = 1
