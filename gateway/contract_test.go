package main

import (
	"encoding/json"
	"os"
	"testing"
)

func TestArtifactContract(t *testing.T) {
	data, err := os.ReadFile("../artifacts/bleach/zanpakuto.json")
	if err != nil {
		t.Fatal(err)
	}

	var items []Zanpakuto
	if err := json.Unmarshal(data, &items); err != nil {
		t.Fatalf("Contract mismatch: %v", err)
	}

	if items[0].Shikai.ReleaseCommand.JP == "" {
		t.Fatal("release_command.jp must exist")
	}
}