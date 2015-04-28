(require :asdf)
(asdf:load-system :cl-opengl)
(asdf:load-system :cl-glu)
(asdf:load-system :cl-glut)

(defparameter *init-dist* 20.0)

(defvar cam-pos '(0.57735026 0.57735026 0.57735026))
(defvar cam-ori '(0.0 0.0 0.0))
(defvar cam-up '(0.0 0.0 1.0))
(defvar cam-dist *init-dist*)
(defvar cam-fovy 45)
(defvar tor-rot 0)
(defvar win-middle)
(defvar win-size)
(defvar norm-n)
(defvar norm-u)
(defvar norm-v)
(defvar key-buf (make-array 190 :element-type 'bit :initial-element 0))
(defvar obj ())
(defvar sn-vn)

(defmacro col255 (cfun &rest r)
  `(,cfun ,@(loop while r collect (/ (pop r) 255.0))))

(defmacro vec255 (r g b a)
  (vector (/ r 255.0) (/ g 255.0) (/ b 255.0) (/ a 255.0)))

(defun rep-n (f n) 
  (if (> n 0)
      (cons (funcall f)
            (rep-n f (- n 1)))
      nil))

(defun poly-obj (o)
  (mapcar (lambda (x) (apply #'gl:vertex x)) o))

(defun vert-obj (a b)
  (when (or a b)
    (apply #'gl:vertex (car a))
    (apply #'gl:vertex (car b))
    (vert-obj (cdr a) (cdr b))))

(defun show-obj (aobj dobj)
  (if dobj
      (let ((adobj (car dobj)))
        (gl:color 0.2 0.1 0.5)
        (gl:with-primitives
          :line-loop
          (poly-obj aobj))
        (gl:color 0.1 0.1 0.1)
        (gl:with-primitives 
          :triangle-strip
          (vert-obj aobj adobj)
          (apply #'gl:vertex (car aobj))
          (apply #'gl:vertex (car adobj)))
        (show-obj (car dobj) (cdr dobj)))
      (progn
        (gl:color 0.2 0.1 0.5)
        (gl:with-primitives
          :line-loop (poly-obj aobj))
        (gl:color 0 0 0)
        (gl:with-primitives
          :polygon (poly-obj aobj)))))

(defun vec-len (v)
  (sqrt (reduce (lambda (x y) (+ x (* y y))) v :initial-value 0)))

(defun normalize (v)
  (mapcar (lambda (p) (/ p (vec-len v))) v))

(defun dot (a b)
  (reduce #'+ (mapcar #'* a b)))

(defun cross (a b)
  (let* ((vect-a (coerce a 'vector))
         (vect-b (coerce b 'vector))
         (a1 (aref vect-a 0)) (a2 (aref vect-a 1)) (a3 (aref vect-a 2))
         (b1 (aref vect-b 0)) (b2 (aref vect-b 1)) (b3 (aref vect-b 2)))
    (list (- (* a2 b3) (* a3 b2))
          (- (* a3 b1) (* a1 b3))
          (- (* a1 b2) (* a2 b1)))))

(defun mat-product (mat3 vec3) ; mat - row major
  (mapcar (lambda (v) (dot v vec3)) mat3))

(defun spin (vec3 cos-a sin-a axis)
  (case axis
        (x (mat-product `((1 0 0)
                          (0 ,cos-a ,(- sin-a))
                          (0 ,sin-a ,cos-a)) vec3))
        (y (mat-product `((,cos-a 0 ,sin-a)
                          (0 1 0)
                          (,(- sin-a) 0 ,cos-a)) vec3))
        (z (mat-product `((,cos-a ,(- sin-a) 0)
                          (,sin-a ,cos-a 0)
                          (0 0 1)) vec3))
        (t '(0 0 1))))

(defun track (x0 y0 x1 y1)
  (let* ((sphere-rad (+ (* (car win-middle) (car win-middle))
                        (* (cdr win-middle) (cdr win-middle))))
         (vec-rot0 (normalize (list x0 y0 (sqrt (- sphere-rad 
                                                   (* x0 x0)
                                                   (* y0 y0))))))
         (vec-rot1 (normalize (list x1 y1 (sqrt (- sphere-rad 
                                                   (* x1 x1)
                                                   (* y1 y1))))))
         (vec-rot-o (cross vec-rot0 vec-rot1))
         (sin-gamma (vec-len vec-rot-o))
         (vec-rot (mapcar (lambda (p) (/ p sin-gamma)) vec-rot-o))
         (cos-gamma (dot vec-rot0 vec-rot1))
         (ref-rot (coerce vec-rot 'vector))
         (temp (sqrt (+ (* (aref ref-rot 1) (aref ref-rot 1))
                        (* (aref ref-rot 2) (aref ref-rot 2)))))
         (cos-alpha (/ (aref ref-rot 2) temp))
         (sin-alpha (/ (aref ref-rot 1) temp))
         (xrot-rot (spin vec-rot cos-alpha sin-alpha 'x))
         (cos-beta (caddr xrot-rot))
         (sin-beta (car xrot-rot))
         (sin-2gam (* 2 sin-gamma cos-gamma))
         (cos-2gam (- (* 2 cos-gamma cos-gamma) 1))
         (spinlist `((,cos-alpha ,sin-alpha x)
                     (,cos-beta ,sin-beta y)
                     (,cos-2gam ,(- sin-2gam) z)
                     (,cos-beta ,(- sin-beta) y)
                     (,cos-alpha ,(- sin-alpha) x)))
         )
    (setf cam-pos
          (mat-product (apply #'mapcar #'list (list norm-u norm-v norm-n))
                       (reduce 
                         (lambda (vec3 sp)
                           (apply #'spin (cons vec3 sp))) spinlist 
                         :initial-value '(0 0 1))))
    (setf cam-up
          (mat-product (apply #'mapcar #'list (list norm-u norm-v norm-n))
                       (reduce 
                         (lambda (vec3 sp)
                           (apply #'spin (cons vec3 sp))) spinlist 
                         :initial-value '(0 1 0))))
    ))

(defun set-project ()
  (gl:matrix-mode :projection)
  (gl:load-identity)
  
  (glu:perspective cam-fovy (/ (car win-size) (cdr win-size)) 0.1 100)
  
  (let* ((temp-pos (mapcar #'+ cam-pos cam-ori))
         (temp-mat (mapcar (lambda (x) (* cam-dist x)) (append temp-pos cam-ori cam-up))))
    (apply #'glu:look-at temp-mat)))

(defun move-camera ()
  (let* ((move-vec-list (list (cons (char-code #\w) norm-v)
                              (cons (char-code #\s) (mapcar #'- norm-v))
                              (cons (char-code #\d) norm-u)
                              (cons (char-code #\a) (mapcar #'- norm-u))
                              (cons (char-code #\q) norm-n)
                              (cons (char-code #\e) (mapcar #'- norm-n))))
         (move-vec (reduce 
                     (lambda (vec3 mlist)
                       (if (eq (aref key-buf (car mlist)) 1)
                           (mapcar #'+ vec3 (cdr mlist))
                           vec3))
                     move-vec-list :initial-value '(0 0 0))))
    (unless (equal move-vec '(0 0 0))
      (setf cam-ori (mapcar #'+ (mapcar (lambda (x) (* x 0.01)) move-vec) cam-ori))
      (set-project))))

(defclass my-window (glut:window)
  ()
  (:default-initargs :width 800 :height 800
                     :title "Common LISP Handler"
                     :mode '(:single :rgb :depth :multisample)))

(defmethod glut:display-window :before ((win my-window))
  (gl:enable :depth-test)
  (gl:depth-func :lequal)
  (gl:hint :perspective-correction-hint :nicest)
  
  (col255 gl:clear-color 180 180 180 0)
  
  (gl:clear-depth 1)
  (gl:shade-model :smooth))

(defmethod glut:display ((win my-window))
  (gl:clear :depth-buffer-bit :color-buffer)
  (move-camera)
  
  (gl:matrix-mode :modelview)
  (gl:load-identity)
  
  ;(gl:light :light0 :position (vector -2 -2 -2 0))
  ;(gl:light :light0 :diffuse (vector 1 1 1 1))
  
  ;(gl:material :front :ambient-and-diffuse
  ;             (vec255 172 188 10 255))
  (gl:color 0 0 0)
  (gl:with-primitives :polygon (poly-obj (car obj)))
  (show-obj (car obj) (cdr obj))
  
  (glut:swap-buffers)
  )

(defmethod glut:reshape ((win my-window) width height)
  (gl:viewport 0 0 width height)
  (setf win-middle (cons (/ width 2) (/ height 2)))
  (setf win-size (cons width height))
  
  (setf norm-n (mapcar (lambda (p) (/ p (vec-len cam-pos))) cam-pos))
  (setf norm-u (normalize (cross cam-up norm-n)))
  (setf norm-v (cross norm-n norm-u))
  
  (set-project)
  
  (gl:matrix-mode :modelview)
  (gl:load-identity)
  )

(defvar fovy-origin)
(defvar dist-origin)
(defvar click-pos ())
(defvar click-pos-mod)
(defvar modbit ())

(defmethod glut:mouse ((win my-window) button state x y)
  (case button
        (:left-button
          (if (eq state :down)
              (progn
                (setf norm-n (mapcar (lambda (p) (/ p (vec-len cam-pos))) cam-pos))
                (setf norm-u (normalize (cross cam-up norm-n)))
                (setf norm-v (cross norm-n norm-u))
                (setf fovy-origin cam-fovy)
                (setf dist-origin cam-dist)
                (setf modbit (glut:get-modifiers))
                (setf click-pos (cons x y))
                (setf click-pos-mod (cons (- x (car win-middle)) 
                                          (- y (cdr win-middle)))))            
              (setf click-pos nil)))
        (:right-button
          (when (eq state :up)
            (let* ((ray0 (multiple-value-bind 
                           (x0 y0 z0) (glu:un-project x (- (cdr win-size) y) 0) (list x0 y0 z0)))
                   (ray1 (multiple-value-bind
                           (x1 y1 z1) (glu:un-project x (- (cdr win-size) y) 1) (list x1 y1 z1)))
                   (ray2 (mapcar #'- ray1 ray0))
                   (rayz (if (eq (caddr ray0) 0.0) 
                             ray0
                             (mapcar #'- ray0
                                     (mapcar (lambda (x) (* x (caddr ray0)
                                                            (/ 1.0 (caddr ray2))))
                                             ray2))))
                   (raynorm (mapcar (lambda (x) (/ x cam-dist)) rayz)))
              
              (setf cam-pos (mapcar #'+ cam-pos (mapcar #'- cam-ori raynorm)))
              (setf cam-ori raynorm)
              (setf cam-dist (* cam-dist (vec-len cam-pos)))
              (setf cam-pos (normalize cam-pos))
              )
            (set-project)
            ))
        (:wheel-down
          (print 'down))
        ))

(defmethod glut:motion ((win my-window) x y)
  (cond
    ((and click-pos (not modbit)) (track (car click-pos-mod) (cdr click-pos-mod) 
                                         (- x (car win-middle)) (- y (cdr win-middle))))
    ((find :active-shift modbit) (setf cam-fovy
                                       (max 10
                                            (+ fovy-origin 
                                               (/ (- y (cdr click-pos))
                                                  (* 0.0625 (cdr win-middle)))))))
    ((find :active-ctrl modbit) (setf cam-dist
                                      (max 0.1 
                                           (+ dist-origin
                                              (/ (- y (cdr click-pos))
                                                 (* 0.125 (cdr win-middle))))))))
  (set-project)
  (glut:post-redisplay))

(defmethod glut:keyboard ((w my-window) key x y)
  (declare (ignore x y))
  (when (> (char-code key) 127) (return-from glut:keyboard nil))
  
  (unless key-buf
    (progn (setf norm-n (mapcar (lambda (p) (/ p (vec-len cam-pos))) cam-pos))
           (setf norm-u (normalize (cross cam-up norm-n)))
           (setf norm-v (cross norm-n norm-u))))
  (case key
        (#\Escape (glut:destroy-current-window)
                  (return-from glut:keyboard))
        (t (setf (aref key-buf (char-code key)) 1))
        )
  (glut:post-redisplay)
  )

(defmethod glut:keyboard-up ((w my-window) key x y)
  (declare (ignore x y))
  (when (> (char-code key) 127) (return-from glut:keyboard-up nil))
  (when (eq key #\f)
    (setf cam-pos (normalize (mapcar #'+ cam-pos cam-ori)))
    (setf cam-dist *init-dist*)
    (setf cam-fovy 45)
    (setf cam-ori '(0.0 0.0 0.0))
    (set-project))
  (setf (aref key-buf (char-code key)) 0)
  (glut:post-redisplay)
  )


(defmethod glut:idle ((win my-window))
  (glut:post-redisplay))



(progn
  (let ((in (open "data.obj")))
    (when in
      (let ((sv (read in)))
        (setf sn-vn sv))
      (setf obj 
            (rep-n 
              (lambda () (rep-n 
                           (lambda () (read in))
                           (cadr sn-vn)))
              (car sn-vn)))
      (close in)))
  (glut:display-window 
    (make-instance 'my-window)))
