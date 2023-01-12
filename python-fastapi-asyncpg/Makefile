build:
	nop

install:
	[ -d venv ] || python -m venv venv
	./venv/bin/pip install -r requirements.txt
	make freeze

freeze:
	./venv/bin/pip freeze > freeze.txt

dev:
	./venv/bin/uvicorn main:app --port 8080 --reload

run:
	./venv/bin/uvicorn main:app --port 8080 --workers `nproc` --log-level error
