build:
	nop

install:
	[ -d venv ] || python -m venv venv
	./venv/bin/pip install -r requirements.txt
	make freeze

freeze:
	./venv/bin/pip freeze > freeze.txt

dev:
	./venv/bin/uvicorn main:app --reload

run:
	./venv/bin/uvicorn main:app --workers `nproc` --log-level error
