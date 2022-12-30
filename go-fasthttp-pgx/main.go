package main

import (
	"context"
	"encoding/json"
	"log"
	"net/http"
	"os"

	"github.com/jackc/pgx/v5/pgxpool"
	"github.com/joho/godotenv"

	"github.com/fasthttp/router"
	"github.com/valyala/fasthttp"
)

type Word struct {
	Id      int
	Title   string
	Content string
}

func main() {
	err := godotenv.Load()
	if err != nil {
		log.Fatal("Error loading .env file")
	}

	conf, err := pgxpool.ParseConfig(os.Getenv("DATABASE_URL"))
	if err != nil {
		panic(err)
	}
	conf.MaxConns = 10

	dbpool, err := pgxpool.NewWithConfig(context.Background(), conf)
	if err != nil {
		panic(err)
	}
	defer dbpool.Close()

	r := router.New()
	r.GET("/", index(dbpool))
	r.GET("/ping", ping)

	log.Print("Starting server at 8080")

	err = fasthttp.ListenAndServe(":8080", r.Handler)
	if err != nil {
		panic(err)
	}
}

func writeError(ctx *fasthttp.RequestCtx, err error) {
	log.Println(err)

	ctx.SetStatusCode(http.StatusInternalServerError)
	ctx.WriteString("Internal Server Error")
}

func index(db *pgxpool.Pool) fasthttp.RequestHandler {
	return func(ctx *fasthttp.RequestCtx) {
		var words []Word

		rows, err := db.Query(
			context.Background(),
			"SELECT id, title, content FROM words LIMIT 100",
		)
		if err != nil {
			writeError(ctx, err)
			return
		}
		defer rows.Close()

		for rows.Next() {
			var word Word
			err = rows.Scan(&word.Id, &word.Title, &word.Content)
			if err != nil {
				break
			}
			words = append(words, word)
		}
		if err != nil {
			writeError(ctx, err)
			return
		}

		bytes, err := json.Marshal(words)
		if err != nil {
			writeError(ctx, err)
			return
		}

		ctx.Write(bytes)
	}
}

func ping(ctx *fasthttp.RequestCtx) {
	ctx.WriteString("OK")
}
