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

$$
\begin{align}
[\text{Prog}] &\to [\text{Stmt}]^* \\
[\text{Stmt}] &\to
\begin{cases}
\text{exit}([\text{Expr}]); \\
\text{let}\space\text{ident} = [\text{Expr}]; \\
\text{if} ([\text{Expr}])[\text{Scope}]\\
[\text{Scope}]
\end{cases} \\
\text{[Scope]} &\to {[\text{Stmt}]^*} \\
[\text{Expr}] &\to
\begin{cases}
[\text{Term}] \\
[\text{BinExpr}]
\end{cases} \\
[\text{BinExpr}] &\to
\begin{cases}
[\text{Expr}] * [\text{Expr}] & \text{prec} = 1 \\
[\text{Expr}] / [\text{Expr}] & \text{prec} = 1 \\
[\text{Expr}] + [\text{Expr}] & \text{prec} = 0 \\
[\text{Expr}] - [\text{Expr}] & \text{prec} = 0 \\
\end{cases} \\
[\text{Term}] &\to
\begin{cases}
\text{intlit} \\
\text{ident} \\
([\text{Expr}])
\end{cases}
\end{align}
$$