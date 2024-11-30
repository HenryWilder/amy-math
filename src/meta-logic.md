## Known restrictions:
$$
\textbf{Exist:} \\[1em]
x \in \set{\dots}
\\\text{can test presence in a set} \\[1em]
x \in (\set{\dots} \cap \set{\dots})
\\\text{can test presence in multiple sets} \\[1em]
A \gets (A \cup \{x\mid\dots\})
\\\text{can insert items into a set} \\[1em]
~\\
\textbf{Do not exist:} \\[1em]
x \notin \set{\dots}
\\\text{cannot test absence in a set} \\[1em]
A \gets (A \setminus \set{\dots})
\\\text{cannot remove items from a set} \\[1em]
{\set{\dots}}'
\\\text{cannot take the opposite of a set} \\[1em]
a = b \quad a \ne b \quad a > b \quad a < b \quad a \ge b \quad a \le b \\
\neg x \quad -x \quad a+b \quad a - b \quad a \cdot b \quad a \div b
\\\text{can neither compare nor operate on items} \\[1em]
\begin{cases}
    x & \text{if } \dots \\
    y & \text{otherwise}
\end{cases}
\\\text{cannot have default case} \\[1em]
x \in A \iff x \notin B
\\\text{inclusion cannot imply exclusion,}
\\\text{exclusion cannot imply inclusion.} \\[1em]
$$

## Definitions

$$
\def\IsTrue{\textrm{IsTrue}}
\IsTrue = \set{\dots} \\[1ex]
\def\IsFalse{\textrm{IsFalse}}
\IsFalse = \set{\dots} \\[1em]
\def\Negate{\textrm{Negate}}
\Negate(x) = \set{} \\[1ex]
\Negate(x\in\IsTrue) \in \IsFalse \\[1ex]
\Negate(x\in\IsFalse) \in \IsTrue \\[1ex]
\\\orange{
    \text{warning: } \\
    \Negate(x \notin (\IsTrue \cup \IsFalse)) \notin (\IsTrue \cup \IsFalse) \\
    \text{if \(x\) is absent from both \(\IsTrue\) and \(\IsFalse\), so is \(\Negate(x)\)}
} \\[1em]
\def\IsEq{\textrm{IsEq}}
\IsEq(a\in\N, b\in\N) = \set{} \\[1ex]
\IsEq(n, n) \in \IsTrue \mid n\in\N \\[1ex]
\cancel{\IsEq(a, b) \in \IsFalse \mid {\color{Red} a \ne b}} \\
{\color{Red}\text{cannot compare items}} \\[1ex]
\cancel{\IsEq(a, b) \in \IsFalse \mid {\color{yellow}\Negate(\IsEq(a, b)) \in \IsTrue}} \\
\orange{
    \text{because \(\Negate(x)\) is absent from all sets \(x\) is absent from,}\\
    \text{\(\Negate(\IsEq(a,b))\) is absent from \(\IsFalse\) in all cases except \(\IsEq(n, n)\).}\\
    \text{\(\Negate(\IsEq(a,b) \mid a \ne b)\) is absent from both \(\IsFalse\) and \(\IsTrue\).}
} \\[1ex]
\cancel{\IsEq(a, b) \in \IsFalse \mid {\color{Red} \IsEq(a, b) \notin \IsTrue}} \\
{\color{Red}\text{cannot test absence from a set}}
$$
