;;; Advent of Code 2017 Exercise 3
;;; Copyright Michael Lazear (C) 2017
;;; upper-left square = (2d)^2 + 1 - average value per depth
;;; lower-right square = (2d + 1)^2 - max value per depth

(define (square x) (* x x))

(define (square-dimensions depth)
    (let* ((length (+ 1 (* 2 depth)))
           (upper-left (+ 1 (square (* 2 depth))))
           (lower-right (square (+ 1 (* 2 depth)))))
    (list (- upper-left (- length 1)) upper-left (- lower-right (- length 1)) lower-right)))

(define square->depth (lambda (sq)
    (ceiling (/ (- (sqrt sq) 1) 2))))

(define (challenge k)
    (let* ((depth (square->depth k))
           (moves (* 2 depth))) ; max possible moves from any depth
        (- moves (apply min (filter (lambda (x) (> x 0)) (map (lambda (x) (- x k)) (square-dimensions depth)))))))
