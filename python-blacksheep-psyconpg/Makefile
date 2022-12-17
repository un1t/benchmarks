build:
	nop

install:
	[ -d venv ] || python -m venv venv
	./venv/bin/pip install -r requirements.txt
	make freeze

freeze:
	pip freeze > freeze.txt

dev:
	uvicorn main:app --reload

run:
	uvicorn main:app --workers `nproc` --log-level error
