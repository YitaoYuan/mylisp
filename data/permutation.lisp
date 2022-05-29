
(define swap (a i j) 
	(begin
		(= t ([] a i))
		([]= a i ([] a j))
		([]= a j t)
	)
)

(define reverse (a l r)
	(if (< l r)
		(begin 
			(swap a l r)
			(reverse a (+ l 1) (- r 1))
		)		
		0
	)
)

(define find_last_bigger (a i n key)
	(if (>= i n)
		(- n 1)
		(if (> ([] a i) key)
			(find_last_bigger a (+ i 1) n key)
			(- i 1)
		)
	)
)

(define next_permutation_loop (a i n)
	(if (< i 0) 
		0
		(if (> ([] a i) ([] a (+ i 1)))
			(next_permutation_loop a (- i 1) n)
			(begin
				(= pos (find_last_bigger a (+ i 1) n ([] a i)))
				(swap a i pos)
				(reverse a (+ i 1) (- n 1))
				1
			)
		)
	)
)

(define next_permutation (a n)
	(next_permutation_loop a (- n 2) n)
)

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

(define print_all_bigger_permutation (a n)
	(begin 
		(printarray a n)
		(if (next_permutation a n)
			(print_all_bigger_permutation a n)
			0
		)
	)
)

(define appendloop (a i n)
	(if (<= i n) 
		(begin 
			(append a i)
			(appendloop a (+ i 1) n)
		)
		0
	)
)

(define main ()
	(begin
		(= n (readint))
		(= a (list))
		(appendloop a 1 n)
		(print_all_bigger_permutation a n)
	)
)

(main)