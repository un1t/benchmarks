package main

import (
	"database/sql"
	"encoding/json"
	"log"
	"net/http"
	"os"

	"github.com/joho/godotenv"

	"gorm.io/driver/postgres"
	"gorm.io/gorm"
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
	databaseUrl := os.Getenv("DATABASE_URL")
	db, err := gorm.Open(postgres.Open(databaseUrl), &gorm.Config{})
	if err != nil {
		panic(err)
	}
	sqlDB, err := db.DB()
	if err != nil {
		panic(err)
	}
	sqlDB.SetMaxIdleConns(10)
	sqlDB.SetMaxOpenConns(10)

	http.HandleFunc("/", index(db))
	http.HandleFunc("/2", second(sqlDB))

	err = http.ListenAndServe(":8080", nil)
	if err != nil {
		panic(err)
	}
}

func writeError(w http.ResponseWriter, err error) {
	log.Println(err)
	w.WriteHeader(http.StatusInternalServerError)
	w.Write([]byte("Internal Server Error"))
}

func index(db *gorm.DB) http.HandlerFunc {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		var words []Word

		err := db.Limit(100).Find(&words).Error
		if err != nil {
			writeError(w, err)
			return
		}

		bytes, err := json.Marshal(words)
		if err != nil {
			writeError(w, err)
			return
		}

		w.Write(bytes)
	})
}

func second(db *sql.DB) http.HandlerFunc {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		var words []Word

		rows, err := db.Query("select id, title, content from words limit 100")
		if err != nil {
			writeError(w, err)
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
			writeError(w, err)
			return
		}

		bytes, err := json.Marshal(words)
		if err != nil {
			writeError(w, err)
			return
		}

		w.Write(bytes)
	})
}
