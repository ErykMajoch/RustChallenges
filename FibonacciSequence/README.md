# Generating the $n^{th}$ Fibonacci Number
Currently I have two functions generating Fibonacci numbers, one using [Binet's Formula](https://en.wikipedia.org/wiki/Fibonacci_number#Relation_to_the_golden_ratio) and one just simple recursion.

## Binet's Formula
To generate the $n^{th}$ number, we can use the following formula
$$F_n=\frac{\varphi^n - \psi^n}{\sqrt{5}}$$
Where $\varphi = \frac{1 + \sqrt{5}}{2} \approx 1.6180339887$ and $\psi = \frac{1 - \sqrt{5}}{2} \approx -0.6180339887$. Using these approximate values for $\varphi$ and $\psi$ and rounding the result of the fraction to a `u32`, I was able to calculate the sequence up to the 42<sup>nd</sup> number.
