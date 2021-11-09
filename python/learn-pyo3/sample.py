def count(text: str, value: str):
    return sum(word == value for line in text.splitlines() for word in line.split(" "))


def count_naive(text: str, value: str):
    total = 0
    for line in text.splitlines():
        for word in line.split(" "):
            if word == value:
                total += 1
    return total


from sample_rs import count_rs, count_iter_rs, count_naive_rs


string = (
    10_000
    * "Albert Einstein (/ˈaɪnstaɪn/ EYEN-styne;[4] German: [ˈalbɛʁt ˈʔaɪnʃtaɪn] (About this soundlisten); 14 March 1879 – 18 April 1955) was a German-born theoretical physicist,[5] widely acknowledged to be one of the greatest physicists of all time. Einstein is best known for developing the theory of relativity, but he also made important contributions to the development of the theory of quantum mechanics. "
)

functions = [
    count,
    count_naive,
    str.count,
    count_rs,
    count_iter_rs,
    count_naive_rs,
]
results = [function(string, "Einstein") for function in functions]
print(results)
print("all functions are equal", all(result == results[0] for result in results))

from timeit import timeit

for function in functions:
    print(
        function.__name__,
        timeit(
            """function(string, "Einstein")""",
            "from __main__ import function, string ",
            number=20,
        ),
    )
