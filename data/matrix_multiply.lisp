(define printarray_loop (a i n)
	(if (< i n)
		(begin 
			(printint ([] a i))
			(printchar 32)
			(printarray_loop a (+ i 1) n)
		)
		(printchar 10)
	)
)

(define printarray (a n)
	(printarray_loop a 0 n)
)

(define printmatrix (a i n)
	(if (< i n)
		(begin
			(printarray ([] a i) n)
			(printmatrix a (+ i 1) n)
		)
		0
	)
)

(define iter3 (Ci Aik Bk j n)
	(if (< j n)
		(begin
			([]= Ci j (+ ([] Ci j) (* Aik ([] Bk j) ) ))
			(iter3 Ci Aik Bk (+ j 1) n)
		)
		0
	)
)

(define iter2 (Ci Ai B k n)
	(if (< k n)
		(begin
			(iter3 Ci ([] Ai k) ([] B k) 0 n)
			(iter2 Ci Ai B (+ k 1) n)
		)
		0
	)
)

(define iter1 (C A B i n)
	(if (< i n)
		(begin
			(iter2 ([] C i) ([] A i) B 0 n)
			(iter1 C A B (+ i 1) n)
		)
		0
	)
)

(define multiply (C A B n)
	(iter1 C A B 0 n)
)

(begin
	(= A (list (list 1 2) (list 3 4)))
	(= B (list (list 1 1) (list 2 4)))
	(= C (list (list 0 0) (list 0 0)))
	(multiply C A B 2)
	(printchar 65)
	(printchar 10)
	(printmatrix A 0 2)
	(printchar 66)
	(printchar 10)
	(printmatrix B 0 2)
	(printchar 67)
	(printchar 10)
	(printmatrix C 0 2)
)
