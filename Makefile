DATA?=res/data.txt

all: run

run: obj
	sbcl --load main.lisp

obj: compile
	./spline_maker $(DATA)

compile:
	cargo build; cp target/debug/spline_maker .

chess:
	sbcl --load chess.lisp

clean:
	cargo clean; rm data.obj spline_maker
