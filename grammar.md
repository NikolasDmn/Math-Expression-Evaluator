$
\begin{align}
[expr] \to \begin{cases}
sin([expr]) \\
cos([expr]) \\
ln([expr]) \\
log([expr]) \\
[binExpr] \\
\text{num}
\end{cases}\\
[binExpr]\to \begin{cases}
\text{[expr] + [expr]},& \text{prec = 0, dir=LTR}\\
\text{[expr] - [expr]},& \text{prec = 0, dir=LTR}\\
\text{[expr] * [expr]},& \text{prec = 1, dir=LTR} \\
\text{[expr] / [expr]},& \text{prec = 1, dir=LTR} \\
\text{[expr]  \^ [expr]},& \text{prec = 2, dir=RTL} 
\end{cases}
\end{align}
$
