import Control.Arrow
import Data.Function
import Data.List (sortOn)

test1 = [(1, 4), (7, 10), (3, 5)]

test2 = [(7, 13), (-7, 19), (15, 16), (-16, 1), (6, 12), (-12, -11), (3, 8)]

main :: IO ()
main =
  test2
    & ( sortOn fst
          >>> mergeRanges
          >>> fmap (\(start, end) -> end - start)
          >>> sum
          >>> print
      )
  where
    mergeRanges intervals =
      case intervals of
        [] -> []
        (l1, h1) : (l2, h2) : rest
          | h1 >= l2 ->
            mergeRanges ((l1, max h1 h2) : rest)
        (interval : rest) -> interval : mergeRanges rest
