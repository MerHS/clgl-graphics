DATA?=res/data.txt

all: compile obj run

compile:
	cargo build; cp target/debug/spline_maker .

obj:
	./spline_maker $(DATA)

run:
	sbcl --load main.lisp

chess:
	sbcl --load chess.lisp

clean:
	cargo clean; rm data.obj spline_maker
