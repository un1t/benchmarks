package main

import (
	"context"
	"encoding/json"
	"log"
	"net/http"
	"os"

	"github.com/jackc/pgx/v5/pgxpool"
	"github.com/joho/godotenv"
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

	dbpool, err := pgxpool.New(context.Background(), os.Getenv("DATABASE_URL"))
	if err != nil {
		panic(err)
	}
	defer dbpool.Close()

	http.HandleFunc("/", index(dbpool))
	http.HandleFunc("/ping", ping)

	err = http.ListenAndServe(":8080", nil)
	if err != nil {
		panic(err)
	}
}

func index(db *pgxpool.Pool) http.HandlerFunc {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		rows, err := db.Query(
			context.Background(),
			"SELECT id, title, content FROM words LIMIT 100",
		)
		if err != nil {
			log.Println(err)
			w.WriteHeader(http.StatusInternalServerError)
			w.Write([]byte("Internal Server Error"))
			return
		}
		defer rows.Close()

		var words []Word

		for rows.Next() {
			var word Word
			err = rows.Scan(&word.Id, &word.Title, &word.Content)
			if err != nil {
				break
			}
			words = append(words, word)
		}
		if err != nil {
			log.Println(err)
			w.WriteHeader(http.StatusInternalServerError)
			w.Write([]byte("Internal Server Error"))
			return
		}

		bytes, err := json.Marshal(words)
		if err != nil {
			log.Println(err)
			w.WriteHeader(http.StatusInternalServerError)
			w.Write([]byte("Internal Server Error"))
			return
		}

		w.Write(bytes)
	})
}

func ping(w http.ResponseWriter, r *http.Request) {
	w.Write([]byte("OK"))
}
