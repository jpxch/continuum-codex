package main

import (
	"encoding/json"
	"log"
	"net/http"
	"os"
)

type MultilingualString struct {
	Jp     string  `json:"jp"`
	Romaji string  `json:"romaji"`
	En     *string `json:"en,omitempty"`
}

type Bankai struct {
	Name MultilingualString `json:"name"`
}

type Shikai struct {
	Name           MultilingualString `json:"name"`
	ReleaseCommand MultilingualString `json:"release_command"`
}

type Zanpakuto struct {
	Id          string             `json:"id"`
	Owner       MultilingualString `json:"owner"`
	SealedName  MultilingualString `json:"sealed_name"`
	Shikai      *Shikai            `json:"shikai"`
	Bankai      *Bankai            `json:"bankai"`
	CanonStatus string             `json:"canon_status"`
}

func main() {
	canonPath := os.Getenv("CODEX_CANON_BLEACH_ZANPAKUTO_JSON")
	if canonPath == "" {
		canonPath = "../artifacts/bleach/zanpakuto.json"
	}

	mux := http.NewServeMux()

	mux.HandleFunc("/health", func(w http.ResponseWriter, _ *http.Request) {
		w.WriteHeader(http.StatusOK)
		_, _ = w.Write([]byte("ok"))
	})

	mux.HandleFunc("/bleach/zanpakuto", func(w http.ResponseWriter, _ *http.Request) {
		raw, err := os.ReadFile(canonPath)
		if err != nil {
			http.Error(w, "failed to read canon artifact", http.StatusInternalServerError)
			return
		}

		var items []Zanpakuto
		if err := json.Unmarshal(raw, &items); err != nil {
			http.Error(w, "failed to parse canon artifact", http.StatusInternalServerError)
			return
		}

		w.Header().Set("Content-Type", "application/json")
		enc := json.NewEncoder(w)
		enc.SetIndent("", "  ")
		_ = enc.Encode(items)
	})

	addr := ":8080"
	log.Printf("gateway listening on %s (canon=%s)", addr, canonPath)
	log.Fatal(http.ListenAndServe(addr, mux))
}